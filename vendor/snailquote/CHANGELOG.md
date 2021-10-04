## v0.3.0

### Changes

* Errors compatible with the `std::error::Error` trait are returned as the
  error type in all results now. Previously all results used `String` for the
  error variant. This change is backwards incompatible, but allows handling
  errors from the library much more easily and correctly. Please consult the
  generated documentation for full details on the new error types.
