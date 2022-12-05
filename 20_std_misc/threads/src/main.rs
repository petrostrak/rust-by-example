// Rust provides a mechanism native OS threads via the `spawn` function, the argument of
// this function is a moving closure.
//
// Those threads will be scheduled by the OS.

use std::thread;

const NTHTHREADS: u32 = 10;

// `main()` is the main thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
