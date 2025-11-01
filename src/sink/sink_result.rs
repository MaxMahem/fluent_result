use crate::internal::Sealed;

/// Extension trait for sinking a variant of a `Result`, leaving an `Option`.
///
/// This is useful for one-sided handling where you want to consume one variant
/// and optionally propagate the other.
pub trait SinkResult<T, E>: Sealed {
    /// Sink the [`Ok`] variant into `sink`, returning the [`Err`] variant, if any.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fluent_result::SinkResult;
    ///
    /// let mut log = String::new();
    ///
    /// let result: Result<u32, &str> = Ok(42);
    /// let option = result.sink_ok(|ok| log.push_str(&format!("ok: {}", ok)));
    /// assert_eq!(log, "ok: 42");
    /// assert_eq!(option, None);
    ///
    /// let mut log = String::new();
    ///
    /// let result: Result<u32, &str> = Err("fail");
    /// let option = result.sink_ok(|_| unreachable!());
    /// assert!(log.is_empty());
    /// assert_eq!(option, Some("fail"));
    /// ```
    fn sink_ok<F>(self, sink: F) -> Option<E>
    where
        F: FnOnce(T);

    /// Sink the [`Err`] variant into `sink`, returning the [`Ok`] variant, if any.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fluent_result::SinkResult;
    ///
    /// let mut log = String::new();
    ///
    /// let result: Result<u32, &str> = Err("fail");
    /// let option = result.sink_err(|err| log.push_str(&format!("err: {}", err)));
    /// assert_eq!(log, "err: fail");
    /// assert_eq!(option, None);
    ///
    /// let mut log = String::new();
    ///
    /// let result: Result<u32, &str> = Ok(42);
    /// let option = result.sink_err(|_| unreachable!());
    /// assert!(log.is_empty());
    /// assert_eq!(option, Some(42));
    /// ```
    fn sink_err<F>(self, sink: F) -> Option<T>
    where
        F: FnOnce(E);
}

impl<T, E> SinkResult<T, E> for Result<T, E> {
    fn sink_ok<F>(self, sink: F) -> Option<E>
    where
        F: FnOnce(T),
    {
        match self {
            Ok(t) => sink(t),
            Err(e) => return Some(e),
        }
        None
    }

    fn sink_err<F>(self, sink: F) -> Option<T>
    where
        F: FnOnce(E),
    {
        match self {
            Ok(t) => return Some(t),
            Err(e) => sink(e),
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sink_ok() {
        let mut log = String::new();

        let result: Result<u32, &str> = Ok(42);
        let option = result.sink_ok(|ok| log.push_str(&format!("ok: {}", ok)));
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
        let option = result.sink_err(|err| log.push_str(&format!("err: {}", err)));
        assert_eq!(log, "err: fail");
        assert_eq!(option, None);

        log.clear();
        let result: Result<u32, &str> = Ok(42);
        let option = result.sink_err(|_| unreachable!());
        assert!(log.is_empty());
        assert_eq!(option, Some(42));
    }
}
