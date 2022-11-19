// There are three types of structures that can be created
// using the `struct` keyword:
// * Tuple structs, which are, basically, named tuples.
// * The classic C structs
// * Unit structs, which are field-less, are useful for generics.

use std::fmt;

// An attribute to hide warnings for unused code.
#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Persons ID:\nName: {}\nAge: {}", self.name, self.age)
    }
}

// A unit struct
struct  Unit;

// A tuple struct
struct  Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and
    // bottom right corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Point { x: top_x, y: top_y } = self.top_left;
        let Point { x: bottom_x, y: bottom_y } = self.bottom_right;
        (top_y - bottom_y) * (bottom_x - top_x)
    }
}

fn square(point: Point, side: f32) -> Rectangle {
    Rectangle { top_left: point, bottom_right: Point { x: side, y: side } }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 36;
    let peter = Person{name,age};

    // Print debug struct
    println!("{:?}", peter);

    // Print fmt::Display struct
    println!("{}", peter);

    // Instantiate a `Point`
    let top_left: Point = Point { x: 5.5, y: 11.0 };

    // Access the fields of the point
    println!("point coordinates: ({} {})", top_left.x, top_left.y);

    let bottom_right: Point = Point { x: 12.3, y: 5.2 };

    // `bottom_right.y` will be the same as `point.y` because we used that
    // field from `point`
    println!("bottom_right coordinates: ({} {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {x: left_edge, y: top_edge} = top_left;

    let _recrangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring)
    println!("Area of rectangle: {:?}", _recrangle.rect_area());

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Return a square
    println!("Square: {:?}", square(top_left, 10.3))
}
