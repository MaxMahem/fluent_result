use std::convert::Infallible;

use fluent_result::expect::UnwrapNever;

#[test]
fn unwrap_never() {
    let result: Result<u32, Infallible> = Ok(42);
    let value = result.unwrap_never();
    assert_eq!(value, 42);
}
