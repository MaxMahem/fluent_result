use fluent_result::expect::expect_none::{dbg, rls};

mod dbg_expect_none {
    use super::dbg::ExpectNone as _;

    #[test]
    fn unwrap_none() {
        None::<u8>.unwrap_none();
    }

    #[test]
    fn expect_none() {
        None::<u8>.expect_none("test");
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "called `Option::unwrap_none()` on a `Some` value")
    )]
    fn unwrap_none_panic() {
        Some(()).unwrap_none();
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic(expected = "test"))]
    fn expect_none_panic() {
        Some(()).expect_none("test");
    }
}

// Tests for rls::ExpectNone (always panics)
mod rls_expect_none {
    use super::rls::ExpectNone as _;

    #[test]
    fn unwrap_none() {
        None::<u8>.unwrap_none();
    }

    #[test]
    fn expect_none() {
        None::<u8>.expect_none("test");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap_none()` on a `Some` value")]
    fn unwrap_none_panic() {
        Some(()).unwrap_none();
    }

    #[test]
    #[should_panic(expected = "test")]
    fn expect_none_panic() {
        Some(()).expect_none("test");
    }
}
