## Creating a Library
Let's create a library, and then see how to link it to another crate.
```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```
```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```
Libraries get prefixed with "lib", and by default they get named after their crate file, but this default name can be overridden by passing the `--crate-name` option to `rustc` or by using the `crate_name` attribute.
## Using a Library
To link a crate to this new library you may use `rustc`'s `--extern` flag. All of its items will then be imported under a module named the same as the library. This module generally behaves the same way as any other module.
```rust
// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
```
```
# Where library.rlib is the path to the compiled library, assumed that it's
# in the same directory here:
$ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```