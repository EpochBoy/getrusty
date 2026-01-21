use std::fs::{self, File};
use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let _greeting_file = match greeting_file_result {
//         Ok(file) => {
//             let contents = fs::read_to_string("hello.txt");
//             println!("File content: {contents:#?}");
//             file
//         },
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file: {error:?}");
//             }
//         },
//     };
// }

// Clean example using unwrap

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}