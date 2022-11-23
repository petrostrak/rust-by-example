// What's a closure?
// In a sentence: a closure is a funtion that can direclty use variables from the
// scope in which this function is defined. This is often described as the closure
// `closing over` or `capturing` variables (the captures).
//
// Syntactically, a closure in Rust is an anonymous function.

fn main() {
    let option = Some(2);

    let x = 3;

    // The closures are capturing the `x` and `y` variables, allowing them to be used
    // while mapping. (To be more convincing, imagine they were only known at runtime)

    // explicit types:
    let new: Option<i32> = option.map(|val: i32| -> i32 { val + x });
    println!("{:?}", new);

    let y = 10;
    // inferred types
    let new2 = option.map(|val| val * y);
    println!("{:?}", new2);
}
