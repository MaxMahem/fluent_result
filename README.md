# Result_Utils
A handful of helpful utilities for handling result types more fluently. 

`IntoOk` and `IntoErr` let you convert a value into an `Ok` or `Err` `Result` variant, via `into_ok` or `into_err`, useful if you hate wrapping and want to postfix ALL THE THINGS!

`IntoSome` provides the same, except for `Option` variant via `into_some`.

`MapTo` provides a convience method for mapping a fixed value to the result of `Ok`, via `map_to`.

`OkOrPanic` lets you return the `Ok` variant of a result, regardless of what the `Err` variant is, panicking if the `Result` is an instance of the `Err` variant. This allows you to convert between Result types with the same succes
type and different error types.

`UnwrapNever` lets you safely unwrap `Infallible` values. This is precisely equivalent to calling `unwrap()`, but with more explicit semantics.
