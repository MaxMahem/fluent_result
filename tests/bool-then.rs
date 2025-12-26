use fluent_result::bool::Then;

#[test]
fn then_err() {
    assert_eq!(true.then_err(42), Err(42));
    assert_eq!(false.then_err(42), Ok(()));
}

#[test]
fn then_err_with() {
    assert_eq!(true.then_err_with(|| 42), Err(42));
    assert_eq!(false.then_err_with(|| 42), Ok(()));
}

#[test]
fn then_result() {
    assert_eq!(true.then_result(42, 0), Ok(42));
    assert_eq!(false.then_result(42, 0), Err(0));
}

#[test]
fn then_result_with() {
    assert_eq!(true.then_result_with(|| 42, || 0), Ok(42));
    assert_eq!(false.then_result_with(|| 42, || 0), Err(0));
}

#[test]
fn then_none() {
    assert_eq!(true.then_none(), None);
    assert_eq!(false.then_none(), Some(()));
}
