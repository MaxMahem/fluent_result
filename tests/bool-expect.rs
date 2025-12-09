macro_rules! test_no_panic {
    ($test_name:ident, $value:expr, $method:ident $(, $msg:expr)?) => {
        #[test]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
}

macro_rules! test_panic {
    (dbg, $test_name:ident, $value:expr, $method:ident, $expected:expr $(, $msg:expr)?) => {
        #[test]
        #[cfg_attr(debug_assertions, should_panic(expected = $expected))]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
    (rls, $test_name:ident, $value:expr, $method:ident, $expected:expr $(, $msg:expr)?) => {
        #[test]
        #[should_panic(expected = $expected)]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
}

mod dbg_expect {
    use fluent_result::bool::dbg::Expect;

    test_no_panic!(assert_true, true, assert_true);
    test_no_panic!(expect_true, true, expect_true, "true");
    test_no_panic!(assert_false, false, assert_false);
    test_no_panic!(expect_false, false, expect_false, "false");

    test_panic!(dbg, assert_true_panic, false, assert_true, "assertion failed: expected `true` but was `false`");
    test_panic!(dbg, expect_true_panic, false, expect_true, "true", "true");
    test_panic!(dbg, assert_false_panic, true, assert_false, "assertion failed: expected `false` but was `true`");
    test_panic!(dbg, expect_false_panic, true, expect_false, "false", "false");
}

// Tests for rls::Expect (always panics)
mod rls_expect {
    use fluent_result::bool::rls::Expect;

    test_no_panic!(assert_true, true, assert_true);
    test_no_panic!(expect_true, true, expect_true, "true");
    test_no_panic!(assert_false, false, assert_false);
    test_no_panic!(expect_false, false, expect_false, "false");

    test_panic!(rls, assert_true_panic, false, assert_true, "assertion failed: expected `true` but was `false`");
    test_panic!(rls, expect_true_panic, false, expect_true, "true", "true");
    test_panic!(rls, assert_false_panic, true, assert_false, "assertion failed: expected `false` but was `true`");
    test_panic!(rls, expect_false_panic, true, expect_false, "false", "false");
}
