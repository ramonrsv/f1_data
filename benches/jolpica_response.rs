use criterion::{BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};

use std::fs;
use std::path::PathBuf;

use serde_json;

use f1_data::jolpica::response::{RaceResult, Response};

static FILENAME: &str = "benches/assets/response_2022_race_results.json";

/// Get the path to [`FILENAME`], relative to the current working directory.
fn get_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(FILENAME)
}

/// Benchmark deserializing a JSON response, in string form, into a [`Response`].
fn bench_deserialize_response(c: &mut Criterion) {
    let content = fs::read_to_string(get_file_path()).unwrap();

    c.bench_function("deserialize_response", |b| b.iter(|| serde_json::from_str::<Response>(&content).unwrap()));
}

/// Benchmark processing a [`Response`], i.e. extracting expected payload, converting types, etc.
fn bench_process_response(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_response");

    let content = fs::read_to_string(get_file_path()).unwrap();
    let response = serde_json::from_str::<Response>(&content).unwrap();

    group.bench_function("into_race_schedules", |b| {
        b.iter_batched(|| response.clone(), |response| response.into_race_schedules(), BatchSize::SmallInput)
    });

    group.bench_function("into_race_schedule", |b| {
        b.iter_batched(|| response.clone(), |response| response.into_race_schedule(), BatchSize::SmallInput)
    });

    group.bench_function("into_many_session_results_for_many_events::<RaceResult>", |b| {
        b.iter_batched(
            || response.clone(),
            |response| response.into_many_session_results_for_many_events::<RaceResult>(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("into_single_session_result_for_single_event::<RaceResult>", |b| {
        b.iter_batched(
            || response.clone(),
            |response| response.into_single_session_result_for_single_event::<RaceResult>(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_deserialize_response, bench_process_response);

criterion_main!(benches);
