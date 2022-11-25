// Variables in Rust do more than just hold data in the stack:
// they also own resources e.g. `Box<T>` owns memory in the heap.
// Rust enforeces RAII (Resource Acquisition Is Initialization), so
// whenever an object goes out of scope, its destructor is called
// and its owned resources are freed. This behavior shields against
// resource leak bugs.

fn create_box() {
    let _box1 = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped!");
    }
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(3i32);
    } //_box3 is destroyed here, and memory gets freed

    // Creating lots of boxes just for fun
    // There's no need to manually free memory
    for _ in 0u32..1000 {
        create_box();
    }

    // The notion of destructor in Rust is provided through the `Drop` trait.
    // The destructor is called when the resource goes out of scope. This trat
    // is not required to be implemented for every type, only implement it for
    // your type if you require its own destructor logic.
    let _x = ToDrop;
    println!("Made a ToDrop!");
} //_box2 is destroyed here, and memory gets freed
