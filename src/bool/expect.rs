/// An extension trait for [`bool`] that provides assertion methods for expected `true` and `false` values.
#[sealed::sealed]
pub trait Expect {
    /// Asserts that the value is `true` with a `msg` assertion message.
    fn expect_true(self, msg: &str);
    /// Asserts that the value is `false` with a `msg` assertion message.
    fn expect_false(self, msg: &str);
}

#[sealed::sealed]
impl Expect for bool {
    #[inline]
    fn expect_true(self, msg: &str) {
        assert!(self, "{}", msg);
    }

    #[inline]
    fn expect_false(self, msg: &str) {
        assert!(!self, "{}", msg);
    }
}
