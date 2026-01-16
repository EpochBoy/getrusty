fn main() {
    print_labeled_measurement(5, 'h');
    run_five();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn run_five() {
    let x = direct_expression();
    let y = statement_then_expression();
    let z = add_one(1);


    println!("The value of x is: {x} / expression");
    println!("The value of y is: {y} / statement");
    println!("The value of z is: {z} / expression + computation");
}

fn direct_expression() -> i32 {
    5                      // Expression
}

fn statement_then_expression() -> i32 {
    let x = 5;             // Statement
    x                      // Expression
}

fn add_one(n: i32) -> i32 {
    n + 1                  // Expression with computation
}


