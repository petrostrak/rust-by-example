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