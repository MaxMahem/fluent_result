# `Fluent_Result`

[![Build](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/fluent_result/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/fluent_result/fluent_result/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/fluent_result/status.svg)](https://deps.rs/repo/github/MaxMahem/fluent_result)
[![codecov](https://codecov.io/gh/MaxMahem/fluent_result/graph/badge.svg?token=G75UDV0DDH)](https://codecov.io/gh/MaxMahem/fluent_result)
![GitHub License](https://img.shields.io/github/license/MaxMahem/fluent_result)

A compact crate offering a suite of extensions providing helpers for manipulating and transforming `Result` and `Option` types fluently,as well as transforming types into `Result` and `Option` types.

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
Transforms `true` into `None` or `Err`, and `false` into `Some(())` or `Ok(())`.

This is useful for ergonomically transforming boolean guards in methods that return `Option<T>` or `Result<T, E>`.

```rust
use fluent_result::bool::Then;

fn foo(number: u32) -> Option<u32> {
    // guard
    (number % 2 == 0).then_none()?;

    // do some more work.

    Some(number)
}
assert_eq!(None, foo(2));


fn bar(number: u32) -> Result<u32, String> {
    // guard
    (number % 2 == 0).then_err("number is even")?;

    // do some more work.

    Ok(number)
}
assert_eq!(Err("number is even".to_string()), bar(2));
```

### `bool::Expect`
Allows you to expect a `true` or `false` value from a bool, with a custom assert message.

```rust
use fluent_result::bool::Expect;

true.expect_true("Panic message if false");
false.exect_false("Panic message if true");
```

### `ExpectNone`
Unwraps the `None` variant of an `Option<T>`. This is useful for validating methods that *should* return `None` but may return `Some` in some cases. For example, when inserting a key value pair that should be unique into a hashmap.

```rust
use std::collections::HashMap;
use fluent_result::expect::ExpectNone;

let mut map = HashMap::new();
map.insert("key", "value").expect_none("key already exists");
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
