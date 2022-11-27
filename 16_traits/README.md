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
