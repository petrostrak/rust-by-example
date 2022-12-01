// The most basic way of handling mixed error types is to just embed them in each other.

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| n * 2))
}

// There are times when we'll want to stop processing on errors (like with `?`) but keep
// going when the `Option` is `None`. A couple of combinators come in handy to swap the
// `Result` and `Option`.
fn double_first_stop_on_error(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| n * 2));
    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // println!("The first double is {:?}", double_first(numbers));
    // println!("The first double is {:?}", double_first(empty)); // Error 1: Empty vector
    // println!("The first double is {:?}", double_first(strings)); // Error 2: non parseable

    println!(
        "The first double is {:?}",
        double_first_stop_on_error(numbers)
    );
    println!(
        "The first double is {:?}",
        double_first_stop_on_error(empty)
    );
    println!(
        "The first double is {:?}",
        double_first_stop_on_error(strings)
    );
}
