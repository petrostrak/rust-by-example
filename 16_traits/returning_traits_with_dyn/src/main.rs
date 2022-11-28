use rand::Rng;

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know
// which one at compilte time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
