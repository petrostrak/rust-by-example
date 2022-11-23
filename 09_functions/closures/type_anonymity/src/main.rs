// When a closure is defined, the compiler implicitly creates a new anonymous struct to store the
// captured variables inside, meanwhile implementing the funcitonality via one of the `traits`: `Fn`,
// `FnMut` or `FnOnce` for this unknown type.
//
// Since this new type is of unknown type, any usage in a function will require generics.

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);
    apply(print);
}
