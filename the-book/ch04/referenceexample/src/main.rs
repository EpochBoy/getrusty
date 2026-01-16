// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// Same functionality but borrowing a reference rather than taking ownership.
// Instead of moving ownership (and returning it back), we borrow with &
//
// How it works:
// - &s1 passes a pointer (memory address) to the function — just 8 bytes, very cheap
// - The actual String data ("hello" on the heap) is NOT copied
// - Both s1 and s point to the same memory location
// - When the function returns, s (the reference) is dropped, but since it
//   doesn't own the data, nothing is deallocated
// - s1 still owns the data and can be used after the function call

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}