use criterion::{criterion_group, criterion_main};
use criterion::{BatchSize, Criterion};

use std::fs;
use std::path::PathBuf;

use serde_json;

use f1_data::ergast::{
    error::{Error, Result},
    get::get_race_results,
    resource::{Filters, Page, Resource},
    response::{Race, RaceResult, Response},
};

fn full_impl(c: &mut Criterion) {
    c.bench_function("get_race_results", |b| b.iter(|| get_race_results(Filters::new().season(2022)).unwrap()));
}

fn stepwise_from_ureq(c: &mut Criterion) {
    let resource = Resource::RaceResults(Filters::new().season(2022));
    let url = resource.to_url_with(Page::with_max_limit());

    let content = ureq::request_url("GET", &url).call().unwrap().into_string().unwrap();
    let response = deserialize(&content);
    let results = process_response(Ok(response.as_ref().unwrap().clone()));

    assert!(results.is_ok());

    c.bench_function("resource_to_url", |b| {
        b.iter(|| Resource::RaceResults(Filters::new().season(2022)).to_url_with(Page::with_max_limit()))
    });

    c.bench_function("ureq_into_string", |b| {
        b.iter(|| ureq::request_url("GET", &url).call().unwrap().into_string().unwrap())
    });

    c.bench_function("deserialize_from_ureq", |b| b.iter(|| deserialize(&content)));

    c.bench_function("process_response_from_ureq", |b| {
        b.iter_batched(
            || Ok(response.as_ref().unwrap().clone()),
            |response| process_response(response).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn stepwise_from_file(c: &mut Criterion) {
    let filename = get_filename();
    let content = read_from_file(&filename);
    let response = deserialize(&content);
    let results = process_response(Ok(response.as_ref().unwrap().clone()));

    assert!(results.is_ok());

    c.bench_function("read_from_file", |b| b.iter(|| read_from_file(&filename)));

    c.bench_function("deserialize_from_file", |b| b.iter(|| deserialize(&content).unwrap()));

    c.bench_function("process_response_from_file", |b| {
        b.iter_batched(
            || Ok(response.as_ref().unwrap().clone()),
            |response| process_response(response).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

fn get_filename() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("benches/data/response_2022_race_results.json")
}

fn read_from_file(filename: &PathBuf) -> String {
    fs::read_to_string(filename).unwrap()
}

fn deserialize(content: &str) -> Result<Response> {
    serde_json::from_str(content).map_err(|e| Error::ParseSerde(e))
}

fn process_response(response: Result<Response>) -> Result<Vec<Race<Vec<RaceResult>>>> {
    response
        .and_then(verify_is_single_page)?
        .mr_data
        .table
        .into_races()?
        .into_iter()
        .map(|race| race.try_map(|payload| payload.into_race_results().map_err(|e| e.into())))
        .collect()
}

fn verify_is_single_page(response: Response) -> Result<Response> {
    if response.mr_data.pagination.is_single_page() {
        Ok(response)
    } else {
        Err(Error::MultiPage)
    }
}

criterion_group!(benches, full_impl, stepwise_from_ureq, stepwise_from_file);
criterion_main!(benches);
