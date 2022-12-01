use std::num::ParseIntError;

// Return the result in case of an error with the `?`
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number + second_number)
}

// Use the `try!` macro instead
fn multiply_with_try(
    first_number_str: &str,
    second_number_str: &str,
) -> Result<i32, ParseIntError> {
    let first_number = r#try!(first_number_str.parse::<i32>());
    let second_number = r#try!(second_number_str.parse::<i32>());

    Ok(first_number + second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    print(multiply_with_try("10", "2"));
    print(multiply_with_try("t", "2"));
}
