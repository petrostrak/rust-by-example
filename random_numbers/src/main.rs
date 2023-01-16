// A lot of applications require random numbers. The rand crate
// is a very popular library in Rust to generate random numbers.
// It supports a wide variety of random number generators and
// distributions, each with a different performance and security
// trade off. For more details, you can read the Rust Rand Book.
// To use the rand crate, just do the following in your Cargo.toml
// file.

extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    // To get a random number, you can simply do the following. The
    // random() is smart enough to know the primitive type it is
    // supposed to generate.
    let i: i32 = rand::random();
    println!("The random i32 is {}", i);

    let x: u8 = rand::random();
    println!("The random u8 is {}", x);

    let x: f64 = rand::random();
    println!("The random f64 is {}", x);

    let x: bool = rand::random();
    println!("The random bool {}", x);

    // What about generating a random number within a range? For that,
    // you need to create a random number generator and call its gen_range()
    // function.
    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(-10.0, 10.0);
    println!("Number from -10. to 10.: {}", y);
    println!("Number from 0 to 9: {}", rng.gen_range(0, 10));

    // Get a series of random numbers
    //
    // In order to get a series of random numbers, you could call the
    // random() function multiple times. But that is slow since every
    // time it needs to instantiate and seed a new random number generator.
    // It is faster to create the generator once and call its gen()
    // function repeatedly.
    for i in 1..10 {
        println!("Random number #{}: {}", i, rng.gen_range(0, 100));
    }

    // The generator can quickly fill an array with random integers.
    let mut arr = [0i32, 9];
    thread_rng().try_fill(&mut arr[..]);
    println!("Random number array {:?}", arr);

    // Another neat feature of the generator is that it can generate random
    // numbers from a probability distribution.
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
}
