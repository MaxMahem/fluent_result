A handful of helpful utilities for handling result types more fluently. 

`IntoOk` and `IntoErr` let you convert a value into an `Ok` or `Err` `Result` variant, via `.into_ok` or `into_err`, useful if you hate wrapping like I do.

`IntoSome` lets you convert a value into a `Some` `Option` variant via `.into_some`.

`OkOrPanic` lets you return the `Ok` variant of a result, regardless of what the `Err` variant is, panicking if the `Result` is an instance of the `Err` variant. This allows you to convert from any two Results with the same success type.

`UnwrapNever` lets you safely unwrap `Infallible` values. This is exactly equivalent to calling `unwrap()`, but with more explicit semantics.
