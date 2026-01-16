fn main() {

    let x1 = String::from("hello");
    let x2 = x1;

    println!("{x2}, world!");

    // Won't work because s1 get's moved onto s2 and s1 thus becomes invalidated (not just a shallow copy)
    // This prevents double-free bugs: if both s1 and s2 pointed to the same heap memory,
    // when they go out of scope, Rust would try to free the same memory twice.
    // By invalidating s1 after the move, only s2 owns the data, so only one free happens.
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1}, world!");


// COPY TYPES (Always same size):
// i8, i16, i32, i64, i128	Fixed size, stack only
// u8, u16, u32, u64, u128	Fixed size, stack only
// isize, usize	Fixed size, stack only
// f32, f64	Fixed size, stack only
// bool	1 byte, stack only
// char	4 bytes, stack only
// &T (references)	Just a pointer (address), stack only
// (i32, bool)	Tuple where ALL elements are Copy
// [i32; 5]	Array where elements are Copy

// MOVE TYPES (Growable/Size can vary):
// String	Has heap data to manage
// Vec<T>	Has heap data to manage
// Box<T>	Heap-allocated
// HashMap<K,V>	Heap data
// Any type with Drop	Needs cleanup
}
