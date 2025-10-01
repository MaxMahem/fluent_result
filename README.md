# Fluent_Result
A compact crate offering a suite of ergonomic postfix helpers for manipulating and transforming Result and Option types fluently.

### Provided Traits
#### into_option
`value.into_option()`
: Transform any value into a `Some`

#### into_result
`value.into_ok()` 
: Transform any value into an `Ok`
`error.into_err()` 
: Transform any value into an `Err`

#### option_map_to
`option.map_to(value)`
: Transform `Option<T>` to `Option<U>`

#### result_map_to
`result.map_to(value)`
: Transform `Result<T, E>` to `Result<U, E>`
`result.map_err_to(error)`
: Transform `Result<T, E1>` to `Result<T, E2>`

#### handle_err
`unit_result.handle_err(sink)` 
: Handles the `Err` variant of a `UnitResult<E>` by sinking it into `sink`. Returns `()`.

#### unwrap_never
`infalliable_result.unwrap_never()`
: Unwraps the `Ok` variant of an `InfalliableResult<T>` without possibility of panic.

#### unwrap_result
`result.unwrap_ok()`
: Transforms a `Result<T, E1>` to a `Result<T, E2>` by unwrapping the success variant and rewrapping, panicing if there is an `Err` variant.
`result.expect_ok(msg)`
: Transforms a `Result<T, E1>` to a `Result<T, E2>` by unwrapping the success variant and rewrapping, panicing with `msg` if there is an `Err` variant.

### Feature Tracing Traits
The optional tracing feature enables additional methods for transforming and Result and Option types via logging there error variants.

#### log_err
`unit_result.log_err(level, "context")`
: Handles the `Err` variant of a `UnitResult<E>` by logging it at a given level with option context with tracing. Returns `()`.

#### ok_log
`result.ok_log(level, "context")`
: Transforms a `Result` into an `Option` by logging the `Err` variant at a given level with optional context.

#### tap_log
`result.tap_ok_log(level, "context")`
: Passes through a `Result` while logging any `Err` variant at a given level with optional context.
`result.tap_err_log(level, "context")`
: Passes through a `Result` while logging any `Ok` variant at a given level with optional context.

