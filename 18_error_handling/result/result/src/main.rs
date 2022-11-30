fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out.
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();

    first_number + second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
    // In the unsuccessful case, parse() leaves us with an error for unwrap() to panic on.
    // Additionally, the panic exits our program and provides an unpleasant error message.
    // To improve the quality of our error message, we should be more specific about the
    // return type and consider explicitly handling the error.
}
