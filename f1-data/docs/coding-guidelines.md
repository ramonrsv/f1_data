# ID Types

ID types, e.g. `DriverID`, should be used uniformly across the crate whenever IDs are required, e.g. `*_id` fields or API parameters. The deserialized JSON responses in `crate::ergast::response` shall also use these ID types for `*_id` fields, e.g. `Driver::driver_id`.