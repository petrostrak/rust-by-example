## Attributes
An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
* conditional compilation of code
* set crate name, version and type (binary or library)
* disable lints (warnings)
* enable compiler features (macros, glob imports, etc.)
* link to a foreign library
* mark functions as unit tests
* mark functions that will be part of a benchmark
* attribute like macros

When attributes apply to a whole crate, their syntax is `#![crate_attribute]`, and when they apply to a module or item, the syntax is `#[item_attribute]` (notice the missing bang !).

Attributes can take arguments with different syntaxes:

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

Attributes can have multiple values and can be separated over multiple lines, too:
```
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```
## dead_code
The compiler provides a `dead_code` lint that will warn about unused functions. An attribute can be used to disable the lint.
```rust
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

fn main() {
    used_function();
}
```
## Crates
The `crate_type` attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library), and the `crate_name` attribute can be used to set the name of the crate.

However, it is important to note that both the `crate_type` and `crate_name` attributes have **no** effect whatsoever when using Cargo, the Rust package manager. Since Cargo is used for the majority of Rust projects, this means real-world uses of `crate_type` and `crate_name` are relatively limited.
```rust
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

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
When the `crate_type` attribute is used, we no longer need to pass the `--crate-type` flag to rustc.
```
$ rustc lib.rs
$ ls lib*
library.rlib
```
## cfg
Configuration conditional checks are possible through two different operators:

* the cfg attribute: #[cfg(...)] in attribute position
* the cfg! macro: cfg!(...) in boolean expressions

While the former enables conditional compilation, the latter conditionally evaluates to `true` or `false` literals allowing for checks at run-time. Both utilize identical argument syntax.

`cfg!`, unlike `#[cfg]`, does not remove any code and only evaluates to true or false. For example, all blocks in an if/else expression need to be valid when `cfg!` is used for the condition, regardless of what `cfg!` is evaluating.
```rust
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```