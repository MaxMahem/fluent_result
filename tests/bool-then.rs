use fluent_result::bool::Then;

#[test]
fn then_err() {
    assert_eq!(true.then_err("error"), Err("error"));
    assert_eq!(false.then_err("error"), Ok(()));
}

#[test]
fn then_err_with() {
    assert_eq!(true.then_err_with(|| "error"), Err("error"));
    assert_eq!(false.then_err_with(|| "error"), Ok(()));
}

#[test]
fn to_result() {
    assert_eq!(true.to_result(42, 0), Ok(42));
    assert_eq!(false.to_result(42, 0), Err(0));
}

#[test]
fn to_result_with() {
    assert_eq!(true.to_result_with(|| 42, || 0), Ok(42));
    assert_eq!(false.to_result_with(|| 42, || 0), Err(0));
}

#[test]
fn then_none() {
    assert_eq!(true.then_none(), None);
    assert_eq!(false.then_none(), Some(()));
}
