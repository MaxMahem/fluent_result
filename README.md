A handful of helpful utilities for handling result types more fluently. 

`IntoOk` and `IntoErr` let you convert a value into an `Ok` or `Err` `Result` variant, via `.into_ok` or `into_err`, useful if you hate wrapping and want to postfix ALL THE THINGS!

`OkOrPanic` lets you return the `Ok` variant of a result, regardless of what the `Err` variant is, panicking if the `Result` is an instance of the `Err` variant.

`UnwrapNever` lets you safely unwrap `Infallible` values. This is precisely equivalent to calling `unwrap()`, but with more explicit semantics.
