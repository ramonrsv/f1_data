# ID Types

ID types, e.g. `DriverID`, will be used as keys for data request functions in the API, but they should not be used as part of the returned data types, e.g. `Driver`. Doing so increases coupling between the modules, increases the complexity of deserializing JSON responses, and potentially increases the complexity of use of the returned data types.
