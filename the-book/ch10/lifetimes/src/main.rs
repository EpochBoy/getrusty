// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }

// fn main() {
//     let x = 5;

//     let r = &x;

//     println!("r: {r}");
// }


// Generics, Trait bounds and lifetimes AIO demo
use std::fmt::Display;

fn main(){
    let result = longest_with_an_announcement("foo", "bar", "Hallo alle leute, looken sie at our wörtern vergleicher");
    println!("String compare result: {result}")
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else if x.len() == y.len() {"They are the same length"} else { y }
}