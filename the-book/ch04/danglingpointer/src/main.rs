fn main() {
    // let reference_to_nothing = dangle();
    let reference_to_nothing= no_dangle();
}

// Dangling, won't work
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// Works
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}