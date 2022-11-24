// Similar to functions, implementations require care to remain regenric.

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Implementation of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// Implementation of GenVal
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
