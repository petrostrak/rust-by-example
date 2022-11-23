// If we declare a function that takes a closure as parameter, then any
// function that satisfies the trait bound of the closure can be passed
// as a parameter.

// Define a function which takes a generic `F` arguments
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("function here!");
}

fn main() {
    // define a closure satisfying the `Fn` bound
    let closure = || println!("closure here!");

    call_me(function);
    call_me(closure);
}
