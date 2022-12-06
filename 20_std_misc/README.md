## Threads
Rust provides a mechanism for spawning native OS threads via the `spawn` function, the argument of this function is a moving closure. These threads will be scheduled by the OS.

Rust makes it very easy to parallelise data processing, without many of the headaches traditionally associated with such an attempt.

The standard library provides great threading primitives out of the box. These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.

The aliasing rules (one writable reference XOR many readable references) automatically prevent you from manipulating state that is visible to other threads. (Where synchronisation is needed, there are synchronisation primitives like `Mutexes` or `Channels`.)

In this example, we will calculate the sum of all digits in a block of numbers. We will do this by parcelling out chunks of the block into different threads. Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sums produced by each thread.

Note that, although we're passing references across thread boundaries, Rust understands that we're only passing read-only references, and that thus no unsafety or data races can occur. Also because the references we're passing have `'static` lifetimes, Rust understands that our data won't be destroyed while these threads are still running. (When you need to share non-`static` data between threads, you can use a smart pointer like Arc to keep the data alive and avoid non-`static` lifetimes.)