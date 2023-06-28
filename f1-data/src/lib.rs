// These are all the lint groups or allowed-by-default lints that are enabled as `deny` in f1_data.
// Lints that are warn-by-default, deny-by-default, or in one of the groups are not included, for
// brevity. As such, it is expected that `cargo build/test/clippy/doc` will be run with
// `-D warnings` in CI workflows. `#![deny(warnings)]` is not included in this list as that is
// an anti-pattern: https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html.
// @todo Move these to Cargo.toml once stabilized: https://github.com/rust-lang/cargo/issues/12115
#![deny(
    // rustc groups
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused,

    // rustc
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences,

    // clippy groups
    clippy::all,
    clippy::cargo,
    clippy::suspicious,

    // rustdoc
    rustdoc::all,
)]
//
// Clippy lints from the `pedantic`, `nursery`, and `restriction` groups are more finicky and
// require a combination of opt-in and opt-out strategies to minimize false positives and verbosity.
#![deny(
    // groups
    clippy::pedantic,
    // clippy::nursery,
    // clippy::restriction,

    // restriction
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::unimplemented,
    clippy::todo,
)]
#![allow(
    // pedantic
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,

    // restriction
    clippy::missing_docs_in_private_items
)]
//
// These lints are temporarily allowed while fixes for associated violations are being worked on.
// Developers can locally change to `warn` to see the warnings - CI would fail due to `-D warnings`.
// @todo Fix the associated violations and remove these lints - list should normally be empty.
#![allow(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    clippy::cargo,
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::redundant_closure_for_method_calls,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::needless_pass_by_value,
    clippy::doc_markdown
)]

// Silence unused-extern-crates lint for criterion; lint doesn't seem to see the use in /benches
#[cfg(test)]
use criterion as _;

pub mod ergast;
pub mod fantasy;
pub mod id;
