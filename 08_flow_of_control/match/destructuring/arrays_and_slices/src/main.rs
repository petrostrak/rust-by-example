// Like tuples, arrays and slices can be destructured this way.

fn main() {
    let array = [-3, 2, 0];

    match array {
        // Binds the second and the third element to the respective variable
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {},", second, third),

        // Single values can be ignored with _
        [-3, _, third] => println!("array[0] = 0, array[2] = {}, array[1] was ignored,", third),

        // We can also bind some and ignore the rest
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and the other ones were ignored", second),

        // Or store them in another array/slice (the type depends in that of the value that is
        // being matched against)
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elemets were {:?},", second, tail),

        // Combining these patterns, we can, for example, bind the first and last values, 
        // and store the rest of them in a single array
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }
}
