use fluent_result::bool::Then;

#[test]
fn test_then_err() {
    assert_eq!(true.then_err(42), Err(42));
    assert_eq!(false.then_err(42), Ok(()));
}

#[test]
fn test_then_none() {
    assert_eq!(true.then_none(), None);
    assert_eq!(false.then_none(), Some(()));
}
