fn main() {
    else_if();
    loop_loop();
    loop_label();
    while_loop();
    while_array();
    for_loop();
    for_range();
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("else_if: number is divisible by 4");
    } else if number % 3 == 0 {
        println!("else_if: number is divisible by 3");
    } else if number % 2 == 0 {
        println!("else_if: number is divisible by 2");
    } else {
        println!("else_if: number is not divisible by 4, 3, or 2");
    }
}

fn loop_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("loop_loop: loop #:{counter}");

        if counter == 6 {
            break counter * 2;
        }
    };

    println!("loop_loop: result is {result}");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("loop_label: count = {count}");
        let mut remaining = 10;

        loop {
            println!("loop_label: remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("loop_label: End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("while_loop: {number}!");

        number -= 1;
    }

    println!("while_loop: LIFTOFF!!!");
}

fn while_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("while_array: the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("for_loop: the value is: {element}");
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("for_range: {number}!");
    }
    println!("for_range: LIFTOFF!!!");
}
