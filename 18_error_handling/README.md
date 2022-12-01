## Error handling
Error handling is the process of handling the possibility of failure. For example, failing to read a file and then continuing to use that *bad* input would clearly be problematic. Noticing and explicitly managing those errors saves the rest of the program from various pitfalls.

There are various ways to deal with errors in Rust, which are described in the following subchapters. They all have more or less subtle differences and different use cases. As a rule of thumb:

An explicit `panic` is mainly useful for tests and dealing with unrecoverable errors. For prototyping it can be useful, for example when dealing with functions that haven't been implemented yet, but in those cases the more descriptive `unimplemented` is better. In tests `panic` is a reasonable way to explicitly fail.

The `Option` type is for when a value is optional or when the lack of a value is not an error condition. For example the parent of a directory - `/` and `C:` don't have one. When dealing with `Option`s, `unwrap` is fine for prototyping and cases where it's absolutely certain that there is guaranteed to be a value. However `expect` is more useful since it lets you specify an error message in case something goes wrong anyway.

When there is a chance that things do go wrong and the caller has to deal with the problem, use `Result`. You can `unwrap` and expect them as well (please don't do that unless it's a test or quick prototype).

For a more rigorous discussion of error handling, refer to the error handling section in the [official book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).
### panic
The simplest error handling mechanism we will see is `panic`. It prints an error message, starts unwinding the stack, and usually exits the program.
### abort and unwind
The panic strategy can be set from the command line by using `abort` or `unwind`.
```
rustc main.rs -C panic=abort
```
## Option & unwrap
An `enum` called `Option<T>` in the `std` library is used when absence is a possibility. It manifests itself as one of two "options":

* `Some(T)`: An element of type T was found
* `None`: No element was found

These cases can either be explicitly handled via `match` or implicitly with `unwrap`. Implicit handling will either return the inner element or `panic`.

Note that it's possible to manually customize `panic` with `expect`, but `unwrap` otherwise leaves us with a less meaningful output than explicit handling.
## Unpacking options with ?
You can unpack `Option`s by using match statements, but it's often easier to use the `?` operator. If `x` is an `Option`, then evaluating `x?` will return the underlying value if `x` is `Some`, otherwise it will terminate whatever function is being executed and return None.
```rust
fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}
```
You can chain many `?s` together to make your code much more readable.
## Combinators:map
`match` is a valid method for handling Options. However, you may eventually find heavy usage tedious, especially with operations only valid with an input. In these cases, [combinators](https://doc.rust-lang.org/reference/glossary.html#combinator) can be used to manage control flow in a modular fashion.

`Option` has a built in method called `map()`, a combinator for the simple mapping of `Some -> Some` and `None -> None`. Multiple `map()` calls can be chained together for even more flexibility.
## Combinators:and_then
`map()` was described as a chainable way to simplify match statements. However, using `map()` on a function that returns an `Option<T>` results in the nested `Option<Option<T>>`. Chaining multiple calls together can then become confusing. That's where another combinator called `and_then()`, known in some languages as flatmap, comes in.

`and_then()` calls its function input with the wrapped value and returns the result. If the `Option` is `None`, then it returns `None` instead.
## Unpacking options and defaults
The is more than one way to unpack an Option and fall back on a default if it is `None`. To choose the one that meets our needs, we need to consider the following:

* do we need eager or lazy evaluation?
* do we need to keep the original empty value intact, or modify it in place?
### or() is chainable, evaluates eagerly, keeps empty value intact
`or()` is chainable and eagerly evaluates its argument, as is shown in the following example. Note that because `or`'s arguments are evaluated eagerly, the variable passed to `or` is moved.
```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` moves its argument.
    // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
    // But the variable named `apple` has been moved regardless, and cannot be used anymore.
    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
 }
```
### or_else() is chainable, evaluates lazily, keeps empty value intact
Another alternative is to use `or_else`, which is also chainable, and evaluates lazily, as is shown in the following example:
```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // Providing kiwi as fallback
    // first_available_fruit: Some(Kiwi)
}
```
### get_or_insert() evaluates eagerly, modifies empty values in place
To make sure that an `Option` contains a value, we can use `get_or_insert` to modify it in place with a fallback value, as is shown in the following example. Note that `get_or_insert` eagerly evaluates its parameter, so variable apple is moved:
```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // my_fruit is: Apple
    // first_available_fruit is: Apple
    //println!("Variable named `apple` is moved: {:?}", apple);
}
```
### get_or_insert_with() evaluates lazily, modifies empty values in place
Instead of explicitly providing a value to fall back on, we can pass a closure to `get_or_insert_with`, as follows:
```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // Providing lemon as fallback
    // my_fruit is: Lemon
    // first_available_fruit is: Lemon

    // If the Option has a value, it is left unchanged, and the closure is not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
    // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
}
```
## Result
`Result` is a richer version of the `Option` type that describes possible error instead of possible absence.

That is, `Result<T, E>` could have one of two outcomes:

* `Ok(T)`: An element `T` was found
* `Err(E)`: An error was found with element `E`

By convention, the expected outcome is `Ok` while the unexpected outcome is `Err`.

Like `Option`, `Result` has many methods associated with it. `unwrap()`, for example, either yields the element `T` or `panic`s. For case handling, there are many combinators between `Result` and `Option` that overlap.

In working with Rust, you will likely encounter methods that return the `Result` type, such as the `parse()` method. It might not always be possible to parse a string into the other type, so `parse()` returns a `Result` indicating possible failure.
### Using Result in main
The `Result` type can also be the return type of the main function if specified explicitly.
If an error occurs within the main function it will return an error code and print a debug representation of the error (using the `Debug` trait). The following example shows such a scenario.
```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```
### map for Result
Generally, we want to return the error to the caller so it can decide what is the right way to respond to errors.

We first need to know what kind of error type we are dealing with. To determine the `Err` type, we look to `parse()`, which is implemented with the `FromStr` trait for `i32`. As a result, the `Err` type is specified as `ParseIntError`.

Luckily, `Option`'s `map`, `and_then`, and many other combinators are also implemented for `Result`. [Result](https://doc.rust-lang.org/std/result/enum.Result.html) contains a complete listing.
### aliases for result
How about when we want to reuse a specific `Result` type many times? Recall that Rust allows us to create `aliases`. Conveniently, we can define one for the specific `Result` in question.

At a module level, creating aliases can be particularly helpful. Errors found in a specific module often have the same `Err` type, so a single alias can succinctly define all associated `Result`s. This is so useful that the `std` library even supplies one: `io::Result!`
### Early returns
Another way to deal with this case analysis is to use a combination of `match` statements and *early returns*.

That is, we can simply stop executing the function and return the error if one occurs. For some, this form of code can be easier to both read and write. 
### Introducing ?
Sometimes we just want the simplicity of `unwrap` without the possibility of a `panic`. Until now, `unwrap` has forced us to nest deeper and deeper when what we really wanted was to get the variable out. This is exactly the purpose of `?`.
Upon finding an `Err`, there are two valid actions to take:

* `panic!` which we already decided to try to avoid if possible
* `return` because an `Err` means it cannot be handled

`?` is *almost* exactly equivalent to an `unwrap` which `return`s instead of panicking on `Err`s. 
### The try! macro
Before there was `?`, the same functionality was achieved with the `try!` macro. The `?` operator is now recommended, but you may still find `try!` when looking at older code.
## Multiple error types
`Result`s interact with other `Results` and `Options` interact with other `Option`s.

Sometimes an `Option` needs to interact with a `Result`, or a `Result<T, Error1>` needs to interact with a Result<T, Error2>`. In those cases, we want to manage our different error types in a way that makes them composable and easy to interact with.