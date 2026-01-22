#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 3.14 };
    let string_and_float = Point { x: "Test", y: 12.12};

    println!("{both_integer:?}");
    println!("{both_float:?}");
    println!("{integer_and_float:?}");
    println!("{string_and_float:?}");

}
