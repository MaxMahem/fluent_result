use fluent_result::into::{IntoOption, IntoResult};

#[test]
fn into_some() {
    let some = 42.into_some();
    assert_eq!(some, Some(42));
}

#[test]
fn into_none() {
    let none: Option<&str> = 42.into_none();
    assert!(none.is_none());
}

#[test]
fn into_ok() {
    let ok: Result<u32, ()> = 42.into_ok();
    assert_eq!(ok, Ok(42));
}

#[test]
fn into_err() {
    let err = "error".into_err::<()>();
    assert_eq!(err, Err("error"));
}
