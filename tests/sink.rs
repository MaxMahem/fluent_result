use fluent_result::{SinkOption, SinkResult};
use std::fmt::Write;

#[test]
fn sink_option() {
    let mut log = String::new();
    Some("oops").sink(|e| log.push_str(e));
    assert_eq!(log, "oops");
}

#[test]
fn sink_ok() {
    let mut log = String::new();

    let result: Result<u32, &str> = Ok(42);
    let option = result.sink_ok(|ok| write!(log, "ok: {ok}").unwrap());
    assert_eq!(log, "ok: 42");
    assert_eq!(option, None);

    log.clear();
    let result: Result<u32, &str> = Err("fail");
    let option = result.sink_ok(|_| unreachable!());
    assert!(log.is_empty());
    assert_eq!(option, Some("fail"));
}

#[test]
fn sink_err() {
    let mut log = String::new();

    let result: Result<u32, &str> = Err("fail");
    let option = result.sink_err(|err| write!(log, "err: {err}").unwrap());
    assert_eq!(log, "err: fail");
    assert_eq!(option, None);

    log.clear();
    let result: Result<u32, &str> = Ok(42);
    let option = result.sink_err(|_| unreachable!());
    assert!(log.is_empty());
    assert_eq!(option, Some(42));
}
