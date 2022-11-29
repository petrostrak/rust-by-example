fn drink_with_abort(beverage: &str) {
    // You should not drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party, run!");
        } else {
            println!("Spit it out!")
        }
    } else {
        println!("Some refreshing {} is all I need!", beverage);
    }
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party, run!");
}

fn drink_with_unwind(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need!", beverage);
    }
}

fn main() {
    // drink_with_abort("water");
    // drink_with_abort("lemonade");

    drink_with_unwind("water");
    drink_with_unwind("lemonade");
}
