window.BENCHMARK_DATA = {
  "lastUpdate": 1790203554722,
  "repoUrl": "https://github.com/ramonrsv/f1_data",
  "entries": {
    "f1_data benchmarks": [
      {
        "commit": {
          "author": {
            "name": "Ramon Sibello",
            "username": "ramonrsv",
            "email": "ramon@sibello.ca"
          },
          "committer": {
            "name": "Ramon Sibello",
            "username": "ramonrsv",
            "email": "ramon@sibello.ca"
          },
          "id": "1abc69894d1daf45383a64e8ffc17946a662b6a7",
          "message": "Track benchmark results on GitHub Pages\n\nUse github-action-benchmark to store benchmark results in the `gh-pages`\nbranch and chart them on GitHub Pages, at\nhttps://ramonrsv.github.io/f1_data/dev/bench/. Benchmarks still only\nrun on the weekly schedule, manually, or locally with `act` via\n`FORCE_RUN_BENCH`, and results are only stored for runs on main, not\nwith `act`. Some benchmarks make requests to the jolpica-f1 API, which\nis too noisy to fail on, so regressions only generate a commit comment.\n\nRun the benchmarks with cargo-criterion, whose bencher output format\nthe action parses. With `cargo bench`, the `--output-format bencher`\nargument would also be passed to the library's libtest harness, which\nrejects it.\n\nGate the whole `bench` job, instead of its individual steps, and move\nthe check that the benchmarks compile into `build_and_test`, which now\nbuilds all targets, i.e. the library, tests, benches, and examples.\n\nAdd a benchmarks badge to README.md, and remove the TODO.md item.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-09-23T22:00:35Z",
          "url": "https://github.com/ramonrsv/f1_data/commit/1abc69894d1daf45383a64e8ffc17946a662b6a7"
        },
        "date": 1790202094272,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_race_results/get_race_results",
            "value": 71419301,
            "range": "± 11592452",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/.read_json::<Response>",
            "value": 755149,
            "range": "± 93158",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/serde_json::from_str::<Response>(.read_to_string())",
            "value": 417600,
            "range": "± 276888",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/serde_json::from_reader::<_, Response>(.into_reader())",
            "value": 767594,
            "range": "± 50221",
            "unit": "ns/iter"
          },
          {
            "name": "read_json/from_file",
            "value": 13432,
            "range": "± 791",
            "unit": "ns/iter"
          },
          {
            "name": "read_json/from_http",
            "value": 62375767,
            "range": "± 5643084",
            "unit": "ns/iter"
          },
          {
            "name": "resource_to_url/filters_none",
            "value": 342,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "resource_to_url/filters_many",
            "value": 1304,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "deserialize_response",
            "value": 2885090,
            "range": "± 24123",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_race_schedules",
            "value": 45266,
            "range": "± 9148",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_race_schedule",
            "value": 41986,
            "range": "± 9303",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_many_races_with_many_session_results::<RaceResult>",
            "value": 1035,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_one_race_with_one_session_result::<RaceResult>",
            "value": 43382,
            "range": "± 10920",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Ramon Sibello",
            "username": "ramonrsv",
            "email": "ramon@sibello.ca"
          },
          "committer": {
            "name": "Ramon Sibello",
            "username": "ramonrsv",
            "email": "ramon@sibello.ca"
          },
          "id": "1abc69894d1daf45383a64e8ffc17946a662b6a7",
          "message": "Track benchmark results on GitHub Pages\n\nUse github-action-benchmark to store benchmark results in the `gh-pages`\nbranch and chart them on GitHub Pages, at\nhttps://ramonrsv.github.io/f1_data/dev/bench/. Benchmarks still only\nrun on the weekly schedule, manually, or locally with `act` via\n`FORCE_RUN_BENCH`, and results are only stored for runs on main, not\nwith `act`. Some benchmarks make requests to the jolpica-f1 API, which\nis too noisy to fail on, so regressions only generate a commit comment.\n\nRun the benchmarks with cargo-criterion, whose bencher output format\nthe action parses. With `cargo bench`, the `--output-format bencher`\nargument would also be passed to the library's libtest harness, which\nrejects it.\n\nGate the whole `bench` job, instead of its individual steps, and move\nthe check that the benchmarks compile into `build_and_test`, which now\nbuilds all targets, i.e. the library, tests, benches, and examples.\n\nAdd a benchmarks badge to README.md, and remove the TODO.md item.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-09-23T22:00:35Z",
          "url": "https://github.com/ramonrsv/f1_data/commit/1abc69894d1daf45383a64e8ffc17946a662b6a7"
        },
        "date": 1790203553844,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_race_results/get_race_results",
            "value": 74909963,
            "range": "± 8389816",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/.read_json::<Response>",
            "value": 944177,
            "range": "± 43038",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/serde_json::from_str::<Response>(.read_to_string())",
            "value": 476431,
            "range": "± 24776",
            "unit": "ns/iter"
          },
          {
            "name": "read_and_process_ureq_response/serde_json::from_reader::<_, Response>(.into_reader())",
            "value": 951153,
            "range": "± 27044",
            "unit": "ns/iter"
          },
          {
            "name": "read_json/from_file",
            "value": 22180,
            "range": "± 404",
            "unit": "ns/iter"
          },
          {
            "name": "read_json/from_http",
            "value": 77067669,
            "range": "± 8316810",
            "unit": "ns/iter"
          },
          {
            "name": "resource_to_url/filters_none",
            "value": 476,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "resource_to_url/filters_many",
            "value": 1762,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "deserialize_response",
            "value": 3908532,
            "range": "± 25518",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_race_schedules",
            "value": 54261,
            "range": "± 11406",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_race_schedule",
            "value": 54038,
            "range": "± 10701",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_many_races_with_many_session_results::<RaceResult>",
            "value": 1170,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "process_response/into_one_race_with_one_session_result::<RaceResult>",
            "value": 56173,
            "range": "± 10527",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}