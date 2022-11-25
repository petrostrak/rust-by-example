## Ownership and moves
Because variables are in charge of freeing their own resources, **resources can only have one owner**. This also prevents resources from being freed more than once. Note that not all variables own resources (e.g. [references](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html)).

When doing assignments (`let x = y;`) or passing function arguments by value (`foo(x)`), the *ownership* of the resources is transferred. In Rust-speaking, this is known as a *move*.

After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.