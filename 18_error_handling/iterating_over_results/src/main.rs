use std::vec;

fn main() {
    // An `Iter::map` operation might fail, for example:
    let vec1 = vec!["tofu", "83", "19"];
    let numbers: Vec<_> = vec1.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // Ignore the failed items with `filter_map()`
    let vec2 = vec!["tofu", "83", "19"];
    let numbers: Vec<_> = vec2
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    // Collect the failed items with `map_err()` and `filter_map()`
    // `map_err()` calls a function with the error, so by adding that
    // to the previous `filter_map` solution, we can save them off to
    // the side while iterating.
    let vec3 = vec!["tofu", "83", "19"];
    let mut errors = vec![];
    let numbers: Vec<_> = vec3
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // Fail the entire operation with `collect()`
    // `Result` implements `FromIter` so that a vector of results (Vec<Result<T, E>>) can be turned
    // into a result with a vector (Result<Vec<T>, E>). Once an `Result::Err` is found, the iteration
    // will terminate.
    let vec4 = vec!["tofu", "83", "19"];
    let numbers: Result<Vec<_>, _> = vec4.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // Collect all valid values and failures with partition()
    let vec5 = vec!["tofu", "83", "19"];
    let (numbers, errors): (Vec<_>, Vec<_>) = vec5
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
