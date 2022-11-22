// The `for in` construct can be used to iterate through an `Iterator`. One of the easiest
// ways to create an iterator is to use the range notation `a..b`. This yields values from 
// `a` (inclusive) to `b` (exclusive) in steps of one. 

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, `a..=b` can be used for a range that is inclusive on both ends.
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for and iterators
    // 
    // The `for in` construct is able to interact with an `Iterator` in several ways. By default
    // the `for` loop will apply the `into_iter` function to the collection. However, this is not
    // the only means of converting collections into iterators.

    // `iter` - This borrows each element of the collection through each iteration. Thus leaving 
    // the collection untouched and avaliable for reuse after the loop.
    let names = vec!["Bob", "Frank", "Peter"];

    for name in names.iter() {
        match name {
            &"Peter" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // `into_iter` - This consumes the collection so that on each iteration the exact data is
    // provided. Once the collection has been consumed, it is no longer available for reuse as
    // it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Peter" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names); // ownership went to `into_iter`.

    // `iter_mut` - This mutably borrows each element of the collection, allowing for the collection
    // to modified in place.
    let mut places = vec!["Athens", "Tokyo", "Hanoi"];
    for place in places.iter_mut() {
        *place = match place {
            &mut "Athens" => "Born there!",
            _ => "Been there",
        }
    }
    println!("places: {:?}", places);
}
