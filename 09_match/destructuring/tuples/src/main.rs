fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third element
        (0, z, y) => println!("First is `0`, `y` is {:?}, and third is {:?}", z, y),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what the are"),
    }
}
