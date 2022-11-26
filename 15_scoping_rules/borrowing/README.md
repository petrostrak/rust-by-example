## Borrowing
Most of the time, we'd like to access data without taking ownership over it. To accomplish this, Rust uses a *borrowing* mechanism. Instead of passing objects by value (`T`), objects can be passed by reference (`&T`).

The compiler statically guarantees (via its borrow checker) that references *always* point to valid objects. That is, while references to an object exist, the object cannot be destroyed.

## Mutability
Mutable data can be mutably borrowed using `&mut T`. This is called a *mutable reference* and
gives read/write access to the borrower. In contrast, `&T` borrows the data via an immutable
reference, and the borrower can read the data but not modify it.

## Aliasing
Data can be immutably borrowed any number of times, but while immutably borrowed, the original data cannot be mutably borrowed. On the other hand, only *one* mutable borrow is allowed at a time. The original data can be borrowed again only *after* the mutable reference has been used for the last time.