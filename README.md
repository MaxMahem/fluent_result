# `Fluent_Result`

[![Build](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/fluent_result/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/fluent_result/fluent_result/index.html)
[![Crates.io](https://img.shields.io/crates/v/fluent_result)](https://crates.io/crates/fluent_result)
[![dependency status](https://deps.rs/repo/github/MaxMahem/fluent_result/status.svg)](https://deps.rs/repo/github/MaxMahem/fluent_result)
[![codecov](https://codecov.io/gh/MaxMahem/fluent_result/graph/badge.svg?token=G75UDV0DDH)](https://codecov.io/gh/MaxMahem/fluent_result)
![GitHub License](https://img.shields.io/github/license/MaxMahem/fluent_result)

A compact crate offering a suite of extensions providing helpers for manipulating and transforming `Result` and `Option` types fluently.

This crate is `no_std` compatible and contains no unsafe code. Some features are gated by `alloc` (enabled by default).

## Provided Traits
### `IntoOption`
Wrap any value in a `Option`. This is equivalent to `Some(value)` and `None`, but may be more readable in long chains.

```rust
use fluent_result::into::IntoOption;

let some = 42.into_some();
assert_eq!(Some(42), some);

// Typically less useful, but included for completeness. 
let none = 42.into_none::<u8>();
assert_eq!(None, none);
```

### `IntoResult`
Wrap any value in a `Result`. This is equivalent to `Ok(value)` and `Err(value)`, but may be more readable in long chains.

```rust
use fluent_result::into::IntoResult;

// if necessary, the error type can be specified
let ok = 42.into_ok::<&str>();
assert_eq!(Ok(42), ok);

// likewise the ok type can be specified
let err = 42.into_err::<&str>();
assert_eq!(Err(42), err);
```

### Sink
Handle a variant of a `Result` or `Option` by sinking it into a `Sink`. 

This is useful for handling a variant by sinking it into a side-effecting function, for example logging. Especially useful for methods that return `Result<(), E>` for example.

See the documentation for brief examples.

### `bool::Then`
Transforms `bool` values into `Option` or `Result` types for easier control flow with the `?` operator, or to replace simple `if` statements.

**Convert to `Option`:**
- `then_none()` - Returns `None` on true, `Some(())` on false (useful for guard clauses)

**Convert to `Result`:**
- `then_err(err)` - Returns `Err(err)` on true, `Ok(())` on false
- `then_err_with(|| err)` - Lazy version of `then_err`
- `then_ok_or(err)` - Returns `Ok(())` on true, `Err(err)` on false
- `then_ok_or_else(|| err)` - Lazy version of `then_ok_or`
- `to_result(on_true, on_false)` - Returns `Ok(on_true)` on true, `Err(on_false)` on false
- `to_result_with(|| on_true, || on_false)` - Lazy version of `to_result`

```rust
use fluent_result::bool::Then;

fn filter_odd(number: u32) -> Option<u32> {
    (number % 2 == 0).then_none()?;
    // do more work
    Some(number)
}
assert_eq!(None, filter_odd(2));

fn reject_even(number: u32) -> Result<u32, &'static str> {
    (number % 2 == 0).then_err("number is even")?;
    // do more work
    Ok(number)
}
assert_eq!(Err("number is even"), reject_even(2));

fn validate_age(age: u32) -> Result<u32, &'static str> {
    (age >= 18).to_result(age, "Must be 18 or older")
}
assert_eq!(Ok(21), validate_age(21));
assert_eq!(Err("Must be 18 or older"), validate_age(16));
```

### `bool::expect`
Provides debug-only (`bool::dbg`) and release-mode (`bool::rls`) assertions for bool values. Each mode offers both `assert_*()` methods with fixed panic messages and `expect_*()` methods with custom messages.

**Debug-only assertions (no-op in release):**
```rust
use fluent_result::bool::dbg::Expect;

true.assert_true();  // Fixed panic message
true.expect_true("Custom panic message");  // Custom panic message
```

**Always-on assertions:**
```rust
use fluent_result::bool::rls::Expect;

true.assert_true();  // Fixed panic message
true.expect_true("Custom panic message");  // Custom panic message
```

### `expect_none`
Provides debug-only (`expect::dbg`) and release-mode (`expect::rls`) assertions for unwrapping the `None` variant of an `Option<T>`. This is useful for validating methods that *should* return `None` but may return `Some` in some cases. For example, when inserting a key value pair that should be unique into a hashmap. Each mode offers both `assert_none()` with a fixed panic message and `expect_none()` with a custom message.

**Debug-only assertions (no-op in release):**
```rust
use std::collections::HashMap;
use fluent_result::expect::dbg::ExpectNone;

let mut map = HashMap::new();
map.insert("key", "value").assert_none();  // Fixed panic message
map.insert("key2", "value2").expect_none("Custom panic message");  // Custom panic message
```

**Always-on assertions:**
```rust
use std::collections::HashMap;
use fluent_result::expect::rls::ExpectNone;

let mut map = HashMap::new();
map.insert("key", "value").assert_none();  // Fixed panic message
map.insert("key2", "value2").expect_none("Custom panic message");  // Custom panic message
```

### `FlattenErr`
Flattens a `Result<Result<T, EInner>, EOuter>` into a `Result<T, NestedError<EInner, EOuter>>`. This is useful when working with nested `Result` types where you want to preserve both the inner and outer error types.

```rust
use fluent_result::nested::{FlattenErr, NestedError};

let result: Result<Result<i32, &str>, i32> = Ok(Ok(1));
let ok = result.flatten_err().expect("should be ok");
assert_eq!(ok, 1);

let result: Result<Result<i32, &str>, i32> = Ok(Err("oops"));
let err = result.flatten_err().expect_err("should be err");
assert_eq!(err, NestedError::Inner("oops"));

let result: Result<Result<i32, &str>, i32> = Err(2);
let err = result.flatten_err().expect_err("should be err");
assert_eq!(err, NestedError::Outer(2));
```

### `BoxErr`
Handles nested `Result` types by boxing errors into a `Box<dyn Error>`. This trait works with `Result`s with up to four layers of nesting, so long as all error types implement `std::error::Error`. This is useful when working with operations that produce nested `Result`s and erasing the error type is acceptable.

If all the error types are the same, consider using `Result::flatten` instead. For results with only two layers of nesting, consider using `FlattenErr::flatten_err`.

This trait requires the `alloc` feature, which is enabled by default.

```rust
use std::error::Error;
use fluent_result::nested::BoxErr;

let result: Result<Result<i32, std::io::Error>, std::io::Error> = Ok(Ok(42));
let boxed: Result<i32, Box<dyn Error>> = result.box_err();
assert_eq!(boxed.unwrap(), 42);

let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Ok(Err(err_io));
let boxed: Result<i32, Box<dyn Error>> = result.box_err();
assert!(boxed.is_err());
```
