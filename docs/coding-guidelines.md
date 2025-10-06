# Module and Directory Structure

1. If the module is self-contained and has no submodules, it should live in a `<module_name>.rs`.
2. If the module contains submodules, all definitions should live in submodules under
   `/<module_name>/*`, and a `/<module_name>/mod.rs` should exist that contains only re-exports from
   the submodules.

# ID Types

ID types, e.g. `DriverID`, should be used uniformly across the crate whenever IDs are required, e.g.
`*_id` fields or API parameters. The deserialized JSON responses in `crate::jolpica::response` shall
also use these ID types for `*_id` fields, e.g. `Driver::driver_id`.
