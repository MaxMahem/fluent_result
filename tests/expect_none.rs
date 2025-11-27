use fluent_result::ExpectNone;

#[test]
#[should_panic(expected = "called `Option::unwrap_none()` on a `Some` value")]
fn unwrap_none_panics_on_some() {
    Some(()).unwrap_none();
}

#[test]
fn unwrap_none_unwraps_none_on_none() {
    assert_eq!(None::<u8>.unwrap_none(), ());
}

#[test]
#[should_panic(expected = "test")]
fn expect_none_panics_on_some() {
    Some(()).expect_none("test");
}

#[test]
fn expect_none_unwraps_none_on_none() {
    assert_eq!(None::<u8>.expect_none("test"), ());
}
