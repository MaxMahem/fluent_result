use fluent_result::BoxErr;

#[test]
fn box_err_nested_no_error() {
    let result: Result<Result<i32, std::io::Error>, std::io::Error> = Ok(Ok(42));
    let boxed: Result<i32, Box<dyn std::error::Error>> = result.box_err();
    assert_eq!(boxed.unwrap(), 42);
}

#[test]
fn box_err_nested_error() {
    let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
    let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Ok(Err(err_io));
    let boxed: Result<i32, _> = result.box_err();
    assert!(boxed.is_err());

    let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Err(std::fmt::Error);
    let boxed: Result<i32, _> = result.box_err();
    assert!(boxed.is_err());
}
