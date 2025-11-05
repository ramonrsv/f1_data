use criterion::{BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};

use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

use serde_json;

use f1_data::{
    jolpica::{
        agent::{Agent, AgentConfigs, MultiPageOption, RateLimiterOption},
        api::{JOLPICA_API_BASE_URL, JOLPICA_API_RATE_LIMIT_QUOTA},
        resource::{Filters, Page, Resource},
        response::Response,
    },
    rate_limiter::RateLimiter,
};

/// Check if benchmarks should use a local jolpica-f1 server, based on `LOCAL_JOLPICA` env variable.
fn is_using_local_jolpica() -> bool {
    std::env::var("LOCAL_JOLPICA").map_or(false, |v| v == "1" || v == "true")
}

/// Get the jolpica-f1 API base URL for benchmarks, based on `LOCAL_JOLPICA` env variable.
///
/// The default base URL is [`JOLPICA_API_BASE_URL`], i.e. the real API base URL. If `LOCAL_JOLPICA`
/// env variable is set, then it returns `"http://localhost:8000/ergast/f1"` for a local server.
fn get_base_url() -> &'static str {
    if is_using_local_jolpica() {
        "http://localhost:8000/ergast/f1"
    } else {
        JOLPICA_API_BASE_URL
    }
}

/// Used to rate limit GET requests in benchmarks to avoid exceeding the jolpica-f1 API rate limits.
///
/// Waiting is done as part of the `setup` setup in [`criterion::Bencher::iter_batched`], so the
/// time spent waiting is not measured as part of the benchmark, which is the desired behavior.
///
/// If [`is_using_local_jolpica()`] is true, then no rate limiting is applied and this is a no-op,
/// otherwise [`RateLimiter::wait_until_ready()`] is called on a shared global rate limiter
/// configured with [`JOLPICA_API_RATE_LIMIT_QUOTA`].
fn rate_limiter_wait_until_ready() {
    static RATE_LIMITER: LazyLock<RateLimiter> = LazyLock::new(|| RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA));

    if !is_using_local_jolpica() {
        RATE_LIMITER.wait_until_ready();
    }
}

/// Instance of [`Agent`] for benchmarks, configured with [`get_base_url()`].
///
/// [`MultiPageOption::Disabled`] is used and no HTTP retries are allowed, as these could interfere
/// with benchmarking individual calls. Rate limiting is disabled since benchmarks need to do any
/// waiting as part of the `setup` step of [`criterion::Bencher::iter_batched`], so that the time
/// spent waiting is not measured as part of the benchmark. See [`rate_limiter_wait_until_ready()`].
static JOLPICA_SP: LazyLock<Agent> = LazyLock::new(|| {
    // Multi-page requests and HTTP retries would interfere with benchmarking individual calls.
    Agent::new(AgentConfigs {
        base_url: get_base_url().to_string(),
        multi_page: MultiPageOption::Disabled,
        http_retries: None,
        rate_limiter: RateLimiterOption::None,
    })
});

/// Sample size for rate-limited GET benchmarks, to keep the total time reasonable given the waits.
///
/// Needs to be used only when [`is_using_local_jolpica()`] is false, i.e. when using the real API.
static RATE_LIMITED_GET_CALL_SAMPLE_SIZE: usize = 20;

static FILTERS: LazyLock<Filters> = LazyLock::new(|| Filters::new().season(2022).round(1));
static RESOURCE: LazyLock<Resource> = LazyLock::new(|| Resource::RaceResults(FILTERS.clone()));
static FILENAME: &str = "benches/assets/response_2022_race_results.json";
static URL: LazyLock<String> = LazyLock::new(|| {
    RESOURCE
        .to_url_with_base_and_opt_page(get_base_url(), Some(Page::with_max_limit()))
        .to_string()
});

/// Benchmark calling [`Agent::get_race_results`], including network overhead, post-processing, etc.
fn bench_get_race_results(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_race_results");

    if !is_using_local_jolpica() {
        group.sample_size(RATE_LIMITED_GET_CALL_SAMPLE_SIZE);
    }

    group.bench_function("get_race_results", |b| {
        b.iter_batched(
            || rate_limiter_wait_until_ready(),
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

    if !is_using_local_jolpica() {
        group.sample_size(RATE_LIMITED_GET_CALL_SAMPLE_SIZE);
    }

    let rate_limited_call = || {
        rate_limiter_wait_until_ready();
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

    if !is_using_local_jolpica() {
        group.sample_size(RATE_LIMITED_GET_CALL_SAMPLE_SIZE);
    }

    let file_path = get_file_path();
    let url = URL.clone();

    group.bench_function("from_file", |b| b.iter(|| fs::read_to_string(&file_path).unwrap()));

    group.bench_function("from_http", |b| {
        b.iter_batched(
            || rate_limiter_wait_until_ready(),
            |_| ureq::get(&url).call().unwrap().body_mut().read_to_string().unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_get_race_results, bench_process_ureq_response, bench_read_json_from_file_vs_http);

criterion_main!(benches);
