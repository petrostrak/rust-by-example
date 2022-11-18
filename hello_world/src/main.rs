fn main() {

    // Print text to the console
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // You can manipulate expressions more easily with block comments
    // than with line comments.
    let x:i32 = 5 + /* 90 + */ 5; 
    println!("Is `x` 10 or 100? x = {}", x);

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{}", 31);

    // Positional arguments can be used. Specifying an integer inside `{}
    // determines which additional argument will be replaced. Arguments srart
    // at 0 immediately after the format string.
    let alice = String::from("Alice");
    let bob: String = String::from("Bob");
    println!("{0}, this is {1}. {1}, this is {0}", alice, bob);

    // As can named arguments.
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after `:`.
    println!("Base 10:              {}",   69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hexadecimal):{:x}", 69420);
    println!("Base 16 (hexadecimal):{:X}", 69420);

    // You can right-justify text with a specified width. This will 
    // output "    1". (Four white spaces and a "1", for a total of 5)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending
    // a '$'
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    let james: String = String::from("James");
    let bond: String = String::from("Bond");
    println!("My name is {0}, {1} {0}", bond, james);

    // Only types that implement fmt::Display can be formatted with `{}`.
    // User-defined types do not implement fmt::Display by default
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement fmt::Display
    println!("This struct `{:?}` won't print.. or?", Structure(3));

    // You can also control the number of decimal places shown.
    let pi: f64 = 3.141592;
    println!("Pi is roughly {0:.3}", pi); // 0 in {} is the argument number
}
