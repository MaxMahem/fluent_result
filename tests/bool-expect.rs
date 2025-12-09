use fluent_result::bool::expect::{dbg, rls};

// Tests for dbg::Expect (debug-mode only panics)
mod dbg_expect {
    use super::dbg::Expect as _;

    #[test]
    fn expect_true() {
        true.expect_true("true");
    }

    #[test]
    fn expect_false() {
        false.expect_false("false");
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic(expected = "true"))]
    fn expect_true_panic() {
        false.expect_true("true");
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic(expected = "false"))]
    fn expect_false_panic() {
        true.expect_false("false");
    }
}

// Tests for rls::Expect (always panics)
mod rls_expect {
    use super::rls::Expect as _;

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
}
