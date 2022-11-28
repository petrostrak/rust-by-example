## Traits
A `trait` is a collection of methods defined for an unknown type: `Self`. They can access other methods declared in the same trait.

Traits can be implemented for any data type. 
## Derive
The compiler is capable of providing basic implementations for some traits via the `#[derive]` attribute. These traits can still be manually implemented if a more complex behavior is required.

The following is a list of derivable traits:

* Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
* `Clone`, to create `T` from `&T` via a copy.
* `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
* `Hash`, to compute a hash from `&T`.
* `Default`, to create an empty instance of a data type.
* `Debug`, to format a value using the `{:?}` formatter.
## Returning Traits with dyn
The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a concrete type. Unlike other languages, if you have a trait like `Animal`, you can't write a function that returns `Animal`, because its different implementations will need different amounts of memory. 

However, there's an easy workaround. Instead of returning a trait object directly, our functions return a `Box` which contains some `Animal`. A `box` is just a reference to some memory in the heap. Because a reference has a statically-known size, and the compiler can guarantee it points to a heap-allocated `Animal`, we can return a trait from our function!

Rust tries to be as explicit as possible whenever it allocates memory on the heap. So if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the `dyn` keyword, e.g. `Box<dyn Animal>`.
## Operator Overloading
In Rust, many of the operators can be overloaded via traits. That is, some operators can be used to accomplish different tasks based on their input arguments. This is possible because operators are syntactic sugar for method calls. For example, the `+` operator in `a + b` calls the `add` method (as in `a.add(b)`). This `add` method is part of the `Add` trait. Hence, the `+` operator can be used by any implementor of the `Add` trait.

A list of the traits, such as `Add`, that overload operators can be found in [core::ops](https://doc.rust-lang.org/core/ops/).
## Drop
The `Drop` trait only has one method: `drop`, which is called automatically when an object goes out of scope. The main use of the `Drop` trait is to free the resources that the implementor instance owns.

`Box`, `Vec`, `String`, `File`, and `Process` are some examples of types that implement the `Drop` trait to free resources. The `Drop` trait can also be manually implemented for any custom data type.
## Iterators
The `Iterator` trait is used to implement iterators over collections such as arrays.

The trait requires only a method to be defined for the `next` element, which may be manually defined in an `impl` block or automatically defined (as in arrays and ranges).

As a point of convenience for common situations, the `for` construct turns some collections into iterators using the [.into_iter()](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) method.