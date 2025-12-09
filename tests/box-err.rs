use fluent_result::nested::BoxErr;

// Single-level Result tests
#[test]
fn box_err_single_ok() {
    let result: Result<i32, std::io::Error> = Ok(42);
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert_eq!(boxed.unwrap(), 42);
}

#[test]
fn box_err_single_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
    let result: Result<i32, std::io::Error> = Err(err_io);
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

// Double-nested Result tests
#[test]
fn box_err_double_nested_ok() {
    let result: Result<Result<i32, std::io::Error>, std::io::Error> = Ok(Ok(42));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert_eq!(boxed.unwrap(), 42);
}

#[test]
fn box_err_double_nested_inner_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
    let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Ok(Err(err_io));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_double_nested_outer_err() {
    let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Err(std::fmt::Error);
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

// Triple-nested Result tests
#[test]
fn box_err_triple_nested_ok() {
    let result: Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error> = Ok(Ok(Ok(123)));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert_eq!(boxed.unwrap(), 123);
}

#[test]
fn box_err_triple_nested_innermost_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
    let result: Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error> = Ok(Ok(Err(err_io)));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_triple_nested_middle_err() {
    let result: Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error> = Ok(Err(std::fmt::Error));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_triple_nested_outer_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::TimedOut);
    let result: Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error> = Err(err_io);
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

// Quadruple-nested Result tests
#[test]
fn box_err_quadruple_nested_ok() {
    #[allow(clippy::type_complexity)]
    let result: Result<
        Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error>,
        std::fmt::Error,
    > = Ok(Ok(Ok(Ok(999))));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert_eq!(boxed.unwrap(), 999);
}

#[test]
fn box_err_quadruple_nested_innermost_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
    #[allow(clippy::type_complexity)]
    let result: Result<
        Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error>,
        std::fmt::Error,
    > = Ok(Ok(Ok(Err(err_io))));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_quadruple_nested_second_err() {
    #[allow(clippy::type_complexity)]
    let result: Result<
        Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error>,
        std::fmt::Error,
    > = Ok(Ok(Err(std::fmt::Error)));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_quadruple_nested_third_err() {
    let err_io = std::io::Error::from(std::io::ErrorKind::InvalidData);
    #[allow(clippy::type_complexity)]
    let result: Result<
        Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error>,
        std::fmt::Error,
    > = Ok(Err(err_io));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}

#[test]
fn box_err_quadruple_nested_outermost_err() {
    #[allow(clippy::type_complexity)]
    let result: Result<
        Result<Result<Result<i32, std::io::Error>, std::fmt::Error>, std::io::Error>,
        std::fmt::Error,
    > = Err(std::fmt::Error);
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert!(boxed.is_err());
}
