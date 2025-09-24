use criterion::{criterion_group, criterion_main};
use criterion::{BatchSize, Criterion};

use std::fs;
use std::path::PathBuf;

use serde_json;
use std::sync::LazyLock;
use url::Url;

use f1_data::{
    ergast::{
        get::get_race_results,
        resource::{Filters, Page, Resource},
        response::{Race, RaceResult, Response},
    },
    error::{Error, Result},
};

static FILTERS: LazyLock<Filters> = LazyLock::new(|| Filters::new().season(2022));
static RESOURCE: LazyLock<Resource> = LazyLock::new(|| Resource::RaceResults(FILTERS.clone()));
static URL: LazyLock<Url> = LazyLock::new(|| RESOURCE.to_url_with(Page::with_max_limit()));

static FILENAME: &str = "benches/assets/response_2022_race_results.json";

/// Benchmark a full call to [`get_race_results`], including network overhead, post-processing, etc.
fn bench_get_race_results(c: &mut Criterion) {
    c.bench_function("get_race_results", |b| b.iter(|| get_race_results(FILTERS.clone()).unwrap()));
}

/// Benchmark different ways to process a [`ureq::Response`] into an [`ergast::Response`].
/// Note that the different functions include the network overhead, since a [`ureq::Response`]
/// keeps the socket open and the body isn't read until one of [`ureq::Response::into_json()`],
/// [`ureq::Response::into_string()`], or [`ureq::Response::into_reader()`] is called.
fn bench_process_ureq_response(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_ureq_response");

    let url = URL.clone();

    group.bench_function("via_into_json", |b| {
        b.iter_batched(
            || ureq::request_url("GET", &url).call().unwrap(),
            |ureq_resp| ureq_resp.into_json::<Response>().unwrap(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("via_into_string", |b| {
        b.iter_batched(
            || ureq::request_url("GET", &url).call().unwrap(),
            |ureq_resp| serde_json::from_str::<Response>(&ureq_resp.into_string().unwrap()).unwrap(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("via_into_reader", |b| {
        b.iter_batched(
            || ureq::request_url("GET", &url).call().unwrap(),
            |ureq_resp| serde_json::from_reader::<_, Response>(ureq_resp.into_reader()).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

/// Get the path to [`FILENAME`], relative to the current working directory.
fn get_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(FILENAME)
}

/// Benchmark the difference between reading a JSON response from file or getting it from HTTP req.
fn bench_read_json_from_file_vs_http(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_json");

    let file_path = get_file_path();
    let url = URL.clone();

    group.bench_function("from_file", |b| b.iter(|| fs::read_to_string(&file_path).unwrap()));

    group.bench_function("from_http", |b| {
        b.iter(|| ureq::request_url("GET", &url).call().unwrap().into_string().unwrap())
    });
}

/// Benchmark deserializing a JSON response, in string form, into an [`ergast::Response`].
fn bench_deserialize_response(c: &mut Criterion) {
    let content = fs::read_to_string(get_file_path()).unwrap();

    c.bench_function("deserialize_response", |b| b.iter(|| serde_json::from_str::<Response>(&content).unwrap()));
}

fn process_response(response: Result<Response>) -> Result<Vec<Race<Vec<RaceResult>>>> {
    response
        .and_then(verify_is_single_page)?
        .table
        .into_races()?
        .into_iter()
        .map(|race| race.try_map(|payload| payload.into_race_results().map_err(|e| e.into())))
        .collect()
}

fn verify_is_single_page(response: Response) -> Result<Response> {
    if response.pagination.is_single_page() {
        Ok(response)
    } else {
        Err(Error::MultiPage)
    }
}

/// Benchmark processing an [`ergast::Response`]...
fn bench_process_response(c: &mut Criterion) {
    let content = fs::read_to_string(get_file_path()).unwrap();
    let response = serde_json::from_str::<Response>(&content).unwrap();

    c.bench_function("process_response", |b| {
        b.iter_batched(|| Ok(response.clone()), |response| process_response(response).unwrap(), BatchSize::SmallInput)
    });
}

criterion_group!(
    benches,
    bench_get_race_results,
    bench_process_ureq_response,
    bench_read_json_from_file_vs_http,
    bench_deserialize_response,
    bench_process_response
);

criterion_main!(benches);
