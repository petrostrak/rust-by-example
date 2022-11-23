// When taking a closure as an input parameter, the closure's complete type must be
// annotated using one of a few `traits`, and they're determined by what the closure
// does with the captured value. In order of decreasing restriction, they are:
// * `Fn`:     the closure uses the captured value by reference(&T)
// * `FnMut`:  the closure uses the captured value by mutable reference (&mut T)
// * `FnOnce`: the closure uses the captured value by value (T)

use std::mem;

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    // The closure take no input and returns nothing.
    F: FnOnce(),
{
    f();
}

// A function which takes a closure and returns an `i32`
fn apply_to_x<F>(f: F, x: i32) -> i32
where
    // The closure takes an `i32` and returns an `i32`
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let _greeting = "hello";
    // A non-copy type.
    // `to_owned()` creates owned data from borrowed one.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzz");

        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;
    let x = 5;
    println!("{} doubled is: {}", x, apply_to_x(double, x));
}
