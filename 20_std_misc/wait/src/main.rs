use std::process::Command;

// $ rustc main.rs && ./main
// # `wait` keeps running for 5 seconds until the `sleep 5` command finishes
// reached end of main

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
