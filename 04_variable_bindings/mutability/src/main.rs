// Variable bindings are immutable by default, but this can be overridden
// using the `mut` modifier.

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // The compiler will throw a detailed diagnostic about mutability errors.
    // _immutable_binding += 1;
}
