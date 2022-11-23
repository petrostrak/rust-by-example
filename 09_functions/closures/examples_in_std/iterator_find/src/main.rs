// Iterator::find is a function which iterates over an iterator and searches for the first value
// which satisfies some condition. If none of the values satisfy the condition, it returns `None`
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not comsumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we need to destructure `&&i32` to `i32`.
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    // Iterator::find gives you a reference to the item. But if you want the index of
    // the item, use Iterator::position.
    let vec3 = vec![1, 9, 3, 3, 13, 2];

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference,
    // we need to destructure `&i32` to `i32`.
    let index_of_first_even_numner = vec3.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_numner, Some(5));

    // `into_iter()` for vecs yields `i32` and `position` does not take a reference. In
    // that case, we do not have to destructure
    let index_of_first_negative_number = vec3.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
