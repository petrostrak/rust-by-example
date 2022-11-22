// A match guard can be added to filter the arm.

#![allow(dead_code)]

enum Temperature {
    Celcius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celcius(35);

    match temperature {
        Temperature::Celcius(t) if t > 30 => println!("{}˚C is above 30 Celcius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celcius(t) => println!("{}˚C is below 30 Celcius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}˚F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}˚F is below 86 Fahrenheit", t),
    }

    // Note that the compiler won't take guard conditions into account when checking if all
    // patterns are covered by the match expression.
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greated than zero"),
        _ => unreachable!("Should never happen."),
    }
}
