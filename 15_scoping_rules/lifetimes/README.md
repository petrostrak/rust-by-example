## Lifetimes

A *lifetime* is a construct the compiler (or more specifically, its *borrow checker*) uses to ensure all borrows are valid. Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. While lifetimes and scopes are often referred to together, they are not the same.

Take, for example, the case where we borrow a variable via `&`. The borrow has a lifetime that is determined by where it is declared. As a result, the borrow is valid as long as it ends before the lender is destroyed. However, the scope of the borrow is determined by where the reference is used.
```rust
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```
## Explicit annotation
The borrow checker uses explicit lifetime annotations to determine how long references should be valid. In cases where lifetimes are not elided, Rust requires explicit annotations to determine what the lifetime of a reference should be. The syntax for explicitly annotating a lifetime uses an apostrophe character as follows:
```rust
foo<'a>
// `foo` has a lifetime parameter `'a`
```
Similar to closures, using lifetimes requires generics. Additionally, this lifetime syntax indicates that the lifetime of `foo` may not exceed that of `'a`. Explicit annotation of a type has the form `&'a T` where `'a` has already been introduced.

In cases with multiple lifetimes, the syntax is similar:
```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```
## Functions
Ignoring elision, function signatures with lifetimes have a few constraints:

* any reference must have an annotated lifetime.
* any reference being returned must have the same lifetime as an input or be `static`.

Additionally, note that returning references without input is banned if it would result in returning references to invalid data.
## Bounds
Just like generic types can be bounded, lifetimes (themselves generic) use bounds as well. The `:` character has a slightly different meaning here, but `+` is the same. Note how the following read:

* `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
* `T: Trait + 'a`: Type `T` must implement trait Trait and *all* references in `T` must outlive `'a`.
## Coersion
A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in. This comes in the form of inferred coercion by the Rust compiler, and also in the form of declaring a lifetime difference.
## Static
Rust has a few reserved lifetime names. One of those is `'static`. You might encounter it in two situations:
```rust
// A reference with 'static lifetime:
let s: &'static str = "hello world";

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}
```
Both are related but subtly different and this is a common source for confusion when learning Rust. Here are some examples for each situation:
### Reference lifetime
As a reference lifetime `'static` indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coerced to a shorter lifetime.

There are two ways to make a variable with `'static` lifetime, and both are stored in the read-only memory of the binary:

* Make a constant with the `static` declaration.
* Make a string literal which has type: `&'static str`.

See the following example for a display of each method:
```rust 
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```
### Trait bound
s a trait bound, it means the type does not contain any non-static references. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.

It's important to understand this means that any owned data always passes a 'static lifetime bound, but a reference to that owned data generally does not:
```rust
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
```