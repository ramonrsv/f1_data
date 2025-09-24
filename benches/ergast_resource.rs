use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use std::sync::LazyLock;

use f1_data::ergast::resource::{Filters, Resource};

static FILTERS_NONE: LazyLock<Filters> = LazyLock::new(|| Filters::none());

static FILTERS_MANY: LazyLock<Filters> = LazyLock::new(|| Filters {
    season: Some(2023),
    round: Some(1),
    driver_id: Some("alonso".into()),
    constructor_id: Some("aston_martin".into()),
    circuit_id: Some("baku".into()),
    qualifying_pos: Some(6),
    grid_pos: Some(6),
    sprint_pos: Some(1),
    finish_pos: Some(4),
    fastest_lap_rank: Some(3),
    finishing_status: Some(1),
});

fn resource_to_url(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_to_url");

    group.bench_with_input(
        BenchmarkId::from_parameter("filters_none"),
        &Resource::RaceResults(FILTERS_NONE.clone()),
        |b, resource| b.iter(|| resource.to_url()),
    );

    group.bench_with_input(
        BenchmarkId::from_parameter("filters_many"),
        &Resource::RaceResults(FILTERS_MANY.clone()),
        |b, resource| b.iter(|| resource.to_url()),
    );
}

criterion_group!(benches, resource_to_url);
criterion_main!(benches);
