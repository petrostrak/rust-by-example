// The `newtype` idiom gives the compiler time guarantees that the right type of value is
// supplied to a program.
//
// For example, an age verification function that checks age in years, *must* be given a
// value of type `Years`.

#[derive(Debug)]
struct Years(i64);

#[derive(Debug)]
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("{:?} years to days: {:?}", age, age_days);
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    // To obtain the `newtype`'s value as the base type, you may use the tuple of
    // destructuring syntax like so:
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple
    let Years(years_as_primitive_1) = years; // Destructuring
}
