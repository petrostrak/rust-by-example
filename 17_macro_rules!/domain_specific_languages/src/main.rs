// Suppose that I want to define a little calculator API. I would like to supply an
// expression and have the output printed to console.
//
// note the two pairs of braces in the macro. The outer ones are part of the syntax
// of macro_rules!, in addition to () or [].

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 /4)
    }
}
