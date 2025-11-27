use fluent_result::{FlattenErr, NestedError};
use std::error::Error;

#[test]
fn flatten_err_ok_ok() {
    let result: Result<Result<i32, &str>, i32> = Ok(Ok(42));
    let flattened = result.flatten_err();
    assert_eq!(flattened, Ok(42));
}

#[test]
fn flatten_err_ok_err() {
    let result: Result<Result<i32, &str>, i32> = Ok(Err("inner error"));
    let flattened = result.flatten_err();
    assert_eq!(flattened, Err(NestedError::Inner("inner error")));
}

#[test]
fn flatten_err_err() {
    let result: Result<Result<i32, &str>, i32> = Err(99);
    let flattened = result.flatten_err();
    assert_eq!(flattened, Err(NestedError::Outer(99)));
}

#[test]
fn nested_error_display_inner() {
    let error: NestedError<&str, &str> = NestedError::Inner("inner message");
    assert_eq!(format!("{}", error), "inner message");
}

#[test]
fn nested_error_display_outer() {
    let error: NestedError<&str, &str> = NestedError::Outer("outer message");
    assert_eq!(format!("{}", error), "outer message");
}

#[test]
fn nested_error_source_inner() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: NestedError<std::io::Error, std::fmt::Error> = NestedError::Inner(io_error);

    let source = error.source();
    assert!(source.is_some());
    assert!(source.unwrap().is::<std::io::Error>());
}

#[test]
fn nested_error_source_outer() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: NestedError<std::fmt::Error, std::io::Error> = NestedError::Outer(io_error);

    let source = error.source();
    assert!(source.is_some());
    assert!(source.unwrap().is::<std::io::Error>());
}
