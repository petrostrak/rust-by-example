// The 'From' and 'Into' traits are inherently linked, and this is actually
// part of its implementation. If you are able to convert type A from type B,
// then it should be easy to believe that we should be able to convert type B
// to type A.
//
// The From trait allows for a type to define how to create itself from another
// type, hence providing a very simple mechanism for converting between several
// types.
//
// For example we can easily convert a str into a String
//
// let my_str = "hello";
// let my_string = String::from(my_str);

#![allow(unused)]

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number { value: i }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num)
}
