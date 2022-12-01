use core::num;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofy", "93", "18"];

    println!("The first double is {}", double_first(numbers));
    println!("The first double is {}", double_first(empty)); // Error 1
    println!("The first double is {}", double_first(strings)); // Error 2
}
