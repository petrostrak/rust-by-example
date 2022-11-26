// Mutable data can be mutably borrowed using `&mut T`. This is called a *mutable reference* and
// gives read/write access to the borrower. In contrast, `&T` borrows the data via an immutable
// reference, and the borrower can read the data but not modify it.

#[derive(Debug, Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// This function takes a reference to a mutable book and changes `year`
// Remember: A *&mut T* gives read/write access to the borrower!
fn new_edition(book: &mut Book, year: u32) {
    book.year = year;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // String literals have type `&'static str`
        author: "Petros Trak",
        title: "Rust by Example",
        year: 2022,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);

    // Borrow a mutable object as mutable
    new_edition(&mut mutabook, 2023u32);

    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook, 1986);
}
