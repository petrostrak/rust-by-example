## marco_rules!
Rust provides a powerful macro system that allows metaprogramming. As you've seen in previous chapters, macros look like functions, except that their name ends with a bang `!`, but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

Macros are created using the `macro_rules!` macro.
```rust
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
```
So why are macros useful?

* Don't repeat yourself. There are many cases where you may need similar functionality in multiple places but with different types. Often, writing a macro is a useful way to avoid repeating code.

* Domain-specific languages. Macros allow you to define special syntax for a specific purpose.

* Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is `println!` which could take any number of arguments, depending on the format string!.
## Syntax
There are three basic ideas:
* Patterns and Designators
* Overloading
* Repetition
### Designators
The arguments of a macro are prefixed by a dollar sign `$` and type annotated with a `designator`.
These are some of the available designatoes:
* `block`
* `expr` is used for expressions
* `ident` is used for variable/function names
* `item`
* `literal` is used for literal constants
* `pat` (pattern)
* `path`
* `stmt` (statement)
* `tt` (token tree)
* `ty` (type)
* `vis` (visibility qualifier)
For a complete list, see the [Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html).
### Overload
Macros can be overloaded to accept different combinations of arguments. In that regard, `macro_rules!` can work similarly to a match block.
### Repeat
Macros can use `+` in the argument list to indicate that an argument may repeat at least once, or `*`, to indicate that the argument may repeat zero or more times.

Surrounding the matcher with `$(...),+` will match one or more expression, separated by commas. Also note that the semicolon is optional on the last case.