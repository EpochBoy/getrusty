use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Closure with immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");


    // Closure with mutable reference
    let mut list = vec![5, 4, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // Closure with immutable reference - taking ownership through move keyword
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Sort by key
    let mut rectangle_list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    rectangle_list.sort_by_key(|r| r.width);
    println!("Rectangles sorted by width: {rectangle_list:#?}");

    rectangle_list.sort_by_key(|r| r.height);
    println!("Rectangles sorted by height: {rectangle_list:#?}");

}