use criterion::{BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use serde_json;
use std::sync::LazyLock;

use f1_data::jolpica::{
    agent::Agent,
    resource::{Filters, Page, Resource},
    response::Response,
};

static FILTERS: LazyLock<Filters> = LazyLock::new(|| Filters::new().season(2022).round(1));
static RESOURCE: LazyLock<Resource> = LazyLock::new(|| Resource::RaceResults(FILTERS.clone()));
static URL: LazyLock<String> = LazyLock::new(|| RESOURCE.to_url_with(Page::with_max_limit()).to_string());

static FILENAME: &str = "benches/assets/response_2022_race_results.json";

static JOLPICA_SP: LazyLock<Agent> = LazyLock::new(|| Agent::default());

/// Duration to wait between GET calls to avoid exceeding the jolpica-f1 API rate limits.
///
/// This is meant to to be used in crude rate limiting since [`Agent`]'s limiting is not available.
/// It is a direct copy of `f1_data::jolpica::tests::util::RATE_LIMIT_DURATION`, which is private.
///
/// Waiting is done as part of the `setup` setup in [`criterion::Bencher::iter_batched`], so the
/// time spent waiting is not measured as part of the benchmark, which is the desired behavior.
static RATE_LIMIT_DURATION: Duration = Duration::from_secs(8);

/// Sample size for GET benchmarks, to keep the total time reasonable given the rate limit waits.
static GET_CALL_SAMPLE_SIZE: usize = 20;

/// Benchmark calling [`Agent::get_race_results`], including network overhead, post-processing, etc.
fn bench_get_race_results(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_race_results");

    group.sample_size(GET_CALL_SAMPLE_SIZE);

    group.bench_function("get_race_results", |b| {
        b.iter_batched(
            || std::thread::sleep(RATE_LIMIT_DURATION),
            |_| JOLPICA_SP.get_race_results(FILTERS.clone()).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

/// Benchmark different ways to read and process a [`ureq::Response`] into a [`Response`].
///
/// Note that [`ureq::get(...).call().unwrap()`][`ureq::RequestBuilder::call`] is done in the
/// `setup` step of [`criterion::Bencher::iter_batched`], so the benchmarks should not measure the
/// network overhead, just the reading of the response body and processing into a [`Response`].
fn bench_process_ureq_response(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_and_process_ureq_response");

    group.sample_size(GET_CALL_SAMPLE_SIZE);

    let rate_limited_call = || {
        std::thread::sleep(RATE_LIMIT_DURATION);
        ureq::get(&URL.to_string()).call().unwrap()
    };

    group.bench_function(".read_json::<Response>", |b| {
        b.iter_batched(
            || rate_limited_call(),
            |ureq_resp| ureq_resp.into_body().read_json::<Response>().unwrap(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("serde_json::from_str::<Response>(.read_to_string())", |b| {
        b.iter_batched(
            || rate_limited_call(),
            |ureq_resp| serde_json::from_str::<Response>(&ureq_resp.into_body().read_to_string().unwrap()).unwrap(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("serde_json::from_reader::<_, Response>(.into_reader())", |b| {
        b.iter_batched(
            || rate_limited_call(),
            |ureq_resp| serde_json::from_reader::<_, Response>(ureq_resp.into_body().into_reader()).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

/// Get the path to [`FILENAME`], relative to the current working directory.
fn get_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(FILENAME)
}

/// Benchmark the difference between reading a JSON response from file or getting it from HTTP req.
///
/// Note that network overhead is included in the "from_http" benchmark, but not in the "from_file"
fn bench_read_json_from_file_vs_http(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_json");

    group.sample_size(GET_CALL_SAMPLE_SIZE);

    let file_path = get_file_path();
    let url = URL.clone();

    group.bench_function("from_file", |b| b.iter(|| fs::read_to_string(&file_path).unwrap()));

    group.bench_function("from_http", |b| {
        b.iter_batched(
            || std::thread::sleep(RATE_LIMIT_DURATION),
            |_| ureq::get(&url).call().unwrap().body_mut().read_to_string().unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_get_race_results, bench_process_ureq_response, bench_read_json_from_file_vs_http);

criterion_main!(benches);
