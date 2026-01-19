// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Same as above, but with tuples.
// Issue: Not clear that we are dealing with height and width

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Same as above using structs, so we can name the values using key:value pairs.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
    
//     // Debug
//     println!("rect1 is {rect1:?}");
//     // Pretty print
//     println!("rect1 is {rect1:#?}");


//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Debug macro instead println, uses stderr instead of stdout

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }


// Same as above but using a method
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}