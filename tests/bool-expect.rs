use fluent_result::bool::Expect;

#[test]
fn expect_true() {
    true.expect_true("true");
}

#[test]
fn expect_false() {
    false.expect_false("false");
}

#[test]
#[should_panic(expected = "true")]
fn expect_true_panic() {
    false.expect_true("true");
}

#[test]
#[should_panic(expected = "false")]
fn expect_false_panic() {
    true.expect_false("false");
}
