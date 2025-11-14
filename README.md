# Fluent_Result

[![Build](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/fluent_result/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/fluent_result/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/fluent_result/fluent_result/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/fluent_result/status.svg)](https://deps.rs/repo/github/MaxMahem/fluent_result)
![GitHub License](https://img.shields.io/github/license/MaxMahem/fluent_result)

A compact crate offering a suite of extensions providing helpers for manipulating and transforming `Result` and `Option` types fluently,as well as transforming types into `Result` and `Option` types.

## Provided Traits
### IntoOption
Wrap any value in a `Option`. This is equivalent to `Some(value)` and `None`, but may be more readable in long chains.

```rust
use fluent_result::IntoOption;

let some = 42.into_some();
assert_eq!(Some(42), some);

// Typically less useful, but included for completeness. 
let none = 42.into_none::<u8>();
assert_eq!(None, none);
```

### IntoResult
Wrap any value in a `Result`. This is equivalent to `Ok(value)` and `Err(value)`, but may be more readable in long chains.

```rust
use fluent_result::IntoResult;

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

### ThenNone and ThenErr
Transforms `true` into `None` or `Err`, and `false` into `Some(())` or `Ok(())`.

This is useful for ergonomically transforming boolean guards in methods that return `Option<T>` or `Result<T, E>`.

```rust
use fluent_result::ThenNone;
use fluent_result::ThenErr;

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

### ExpectNone
Unwraps the `None` variant of an `Option<T>`. This is useful for validating methods that *should* return `None` but may return `Some` in some cases. For example, when inserting a key value pair that should be unique into a hashmap.

```rust
use std::collections::HashMap;
use fluent_result::ExpectNone;

let mut map = HashMap::new();
map.insert("key", "value").expect_none("key already exists");
```
