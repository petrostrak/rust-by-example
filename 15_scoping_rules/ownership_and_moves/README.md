## Ownership and moves
Because variables are in charge of freeing their own resources, **resources can only have one owner**. This also prevents resources from being freed more than once. Note that not all variables own resources (e.g. [references](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html)).

When doing assignments (`let x = y;`) or passing function arguments by value (`foo(x)`), the *ownership* of the resources is transferred. In Rust-speaking, this is known as a *move*.

After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.

## Partial moves
Within the destructuring of a single variable, both `by-move`
and `by-reference` pattern bindings can be used at the same
time. Doing this will result in a *partial move* of the variable,
which means that parts of the variable will be moved while other
parts stay. In such a case, the parent variable cannot be used
afterwards as a whole, however the parts that are only referenced
(and not moved) can still be used.