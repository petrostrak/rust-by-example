## Threads
Rust provides a mechanism for spawning native OS threads via the `spawn` function, the argument of this function is a moving closure. These threads will be scheduled by the OS.

Rust makes it very easy to parallelise data processing, without many of the headaches traditionally associated with such an attempt.

The standard library provides great threading primitives out of the box. These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.

The aliasing rules (one writable reference XOR many readable references) automatically prevent you from manipulating state that is visible to other threads. (Where synchronisation is needed, there are synchronisation primitives like `Mutexes` or `Channels`.)

In this example, we will calculate the sum of all digits in a block of numbers. We will do this by parcelling out chunks of the block into different threads. Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sums produced by each thread.

Note that, although we're passing references across thread boundaries, Rust understands that we're only passing read-only references, and that thus no unsafety or data races can occur. Also because the references we're passing have `'static` lifetimes, Rust understands that our data won't be destroyed while these threads are still running. (When you need to share non-`static` data between threads, you can use a smart pointer like Arc to keep the data alive and avoid non-`static` lifetimes.)
## Channels
Rust provides asynchronous `channels` for communication between threads. Channels allow a unidirectional flow of information between two end-points: the `Sender` and the `Receiver`.
## Path
The `Path` struct represents file paths in the underlying filesystem. There are two flavors of `Path`: `posix::Path`, for UNIX-like systems, and `windows::Path`, for Windows. The prelude exports the appropriate platform-specific `Path` variant.

A Path can be created from an `OsStr`, and provides several methods to get information from the file/directory the path points to.

A `Path` is immutable. The owned version of `Path` is `PathBuf`. The relation between `Path` and `PathBuf` is similar to that of `str` and `String`: a `PathBuf` can be mutated in-place, and can be dereferenced to a `Path`.

Note that a `Path` is not internally represented as an UTF-8 string, but instead is stored as a vector of bytes (`Vec<u8>`). Therefore, converting a `Path` to a `&str` is not free and may fail (an `Option` is returned).
## File I/O
The `File` struct represents a file that has been opened (it wraps a file descriptor), and gives read and/or write access to the underlying file.

Since many things can go wrong when doing file I/O, all the `File` methods return the `io::Result<T>` type, which is an alias for `Result<T, io::Error>`.

This makes the failure of all I/O operations *explicit*. Thanks to this, the programmer can see all the failure paths, and is encouraged to handle them in a proactive manner.
### open
The `open` function can be used to open a file in read-only mode.

A `File` owns a resource, the file descriptor and takes care of closing the file when it is droped.
### create
The `create` function opens a file in write-only mode. If the file already existed, the old content is destroyed. Otherwise, a new file is created.
### read_lines
The method `lines()` returns an iterator over the lines of a file.

`File::open` expects a generic, `AsRef<Path>`. That's what `read_lines()` expects as input.
## Child processes
The `process::Output` struct represents the output of a finished child process, and the `process::Command` struct is a process builder.