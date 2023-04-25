# Module and Directory Structure

1. If the module is self-contained and has no submodules, it should live in a file `<module_name>.rs`.
2. If the module contains submodules, all definitions should live in submodules under `/<module_name>/*`, and a `/<module_name>/mod.rs` should exist that contains only re-exports from the submodules.
