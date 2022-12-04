## Box, stack and heap
All values in Rust are stack allocated by default. Values can be *boxed* (allocated on the heap) by creating a `Box<T>`. A box is a smart pointer to a heap allocated value of type `T`. When a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed.

Boxed values can be dereferenced using the `*` operator; this removes one layer of indirection.
## Vectors
Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they can grow or shrink at any time. A vector is represented using 3 parameters: 

* pointer to the data
* length
* capacity

The capacity indicates how much memory is reserved for the vector. The vector can grow as long as the length is smaller than the capacity. When this threshold needs to be surpassed, the vector is reallocated with a larger capacity.

More `Vec` methods can be found under the [std::vec](https://doc.rust-lang.org/std/vec/) module.
## Strings
There are two types of strings in Rust: `String` and `&str`.

A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed to always be a valid UTF-8 sequence. `String` is heap allocated, growable and not null terminated.

`&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and can be used to view into a `String`, just like `&[u8]` is a view into `Vec<T>`.

More `str/String` methods can be found under the [std::str](https://doc.rust-lang.org/std/str/) and [std::string](https://doc.rust-lang.org/std/string/) modules.
## Literals and escapes
There are multiple ways to write string literals with special characters in them. All result in a similar `&str` so it's best to use the form that is the most convenient to write. Similarly there are multiple ways to write byte string literals, which all result in `&[u8; N]`.

Generally special characters are escaped with a backslash character: `\`. This way you can add any character to your string, even unprintable ones and ones that you don't know how to type. If you want a literal backslash, escape it with another one: `\\`

String or character literal delimiters occuring within a literal must be escaped: `"\""`, `'\''`.

For conversions between character encodings check out the [encoding](https://crates.io/crates/encoding) crate.

A more detailed listing of the ways to write string literals and escape characters is given in the ['Tokens' chapter](https://doc.rust-lang.org/reference/tokens.html) of the Rust Reference.
## Option
Sometimes it's desirable to catch the failure of some parts of a program instead of calling `panic!`; this can be accomplished using the `Option` enum.

The `Option<T>` enum has two variants:

* `None`, to indicate failure or lack of value, and
* `Some(value)`, a tuple struct that wraps a `value` with type `T`.
## Result
We've seen that the `Option` enum can be used as a return value from functions that may fail, where `None` can be returned to indicate failure. However, sometimes it is important to express why an operation failed. To do this we have the `Result` enum.
The `Result<T, E>` enum has two variants:

* `Ok(value)` which indicates that the operation succeeded, and wraps the value returned by the operation. (`value` has type `T`)
* `Err(why)`, which indicates that the operation failed, and wraps why, which (hopefully) explains the cause of the failure. (`why` has type `E`)
## ?
Chaining results using match can get pretty untidy; luckily, the `?` operator can be used to make things pretty again. `?` is used at the end of an expression returning a `Result`, and is equivalent to a match expression, where the `Err(err)` branch expands to an early `return Err(From::from(err))`, and the `Ok(ok)` branch expands to an `ok` expression.

Be sure to check the [documentation](https://doc.rust-lang.org/std/result/index.html), as there are many methods to map/compose `Result`.
## panic!
The `panic!` macro can be used to generate a panic and start unwinding its stack. While unwinding, the runtime will take care of freeing all the resources *owned* by the thread by calling the destructor of all its objects.

Since we are dealing with programs with only one thread, `panic!` will cause the program to report the panic message and exit.
## HashMap
Where vectors store values by an integer index, `HashMaps` store values by key. `HashMap` keys can be booleans, integers, strings, or any other type that implements the `Eq` and `Hash` traits. 

Like vectors, `HashMap`s are growable, but HashMaps can also shrink themselves when they have excess space. You can create a HashMap with a certain starting capacity using `HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a HashMap with a default initial capacity (recommended).

For more information on how hashing and hash maps (sometimes called hash tables) work, have a look at [Hash Table Wikipedia](https://en.wikipedia.org/wiki/Hash_table).
## Alternate/custom key types
Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. This includes:

* `bool` (though not very useful since there is only two possible keys)
* `int`, `uint`, and all variations thereof
* `String` and `&str` (protip: you can have a `HashMap` keyed by `String` and call `.get()` with an `&str`)

Note that `f32` and `f64` do not implement Hash, likely because floating-point precision errors would make using them as hashmap keys horribly error-prone.

All collection classes implement `Eq` and `Hash` if their contained type also respectively implements `Eq` and `Hash`. For example, `Vec<T>` will implement `Hash` if `T` implements `Hash`.

You can easily implement `Eq` and `Hash` for a custom type with just one line: `#[derive(PartialEq, Eq, Hash)]`. The compiler will do the rest. 
## HashSet
Consider a `HashSet` as a `HashMap` where we just care about the keys ( `HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`).

"What's the point of that?" you ask. "I could just store the keys in a `Vec`."

A `HashSet`'s unique feature is that it is guaranteed to not have duplicate elements. That's the contract that any set collection fulfills. `HashSet` is just one implementation. (see also: [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)).

If you insert a value that is already present in the `HashSet`, (i.e. the new value is equal to the existing and they both have the same hash), then the new value will replace the old.

This is great for when you never want more than one of something, or when you want to know if you've already got something.

But sets can do more than that.

Sets have 4 primary operations (all of the following calls return an iterator):

* `union`: get all the unique elements in both sets.

* `difference`: get all the elements that are in the first set but not the second.

* `intersection`: get all the elements that are only in both sets.

* `symmetric_difference`: get all the elements that are in one set or the other, but not both.
