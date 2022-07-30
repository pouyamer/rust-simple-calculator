// Calculator

// 1. gets the first number
// 2. gets the second number
// 3. choose the operation (1: +, 2: -, 3: *, 4: /, 5: %)

// get the io module
use std::io;

fn get_input(message: &str) -> i32 {
    let mut variable = String::new();

    println!("{}", message);
    io::stdin()
        .read_line(&mut variable)
        .expect("Couldn't get the input.");

    return variable.trim().parse().expect("Please enter a number.");
}

fn get_sign(operation_number: i32) -> &'static str {
    match operation_number {
        1 => "+",
        2 => "-",
        3 => "*",
        4 => "/",
        5 => "%",
        _ => "",
    }
}

fn calculate_result(first_number: i32, second_number: i32, operation_number: i32) -> i32 {
    if second_number == 0 && operation_number == 4 {
        // throw an error if the second number is 0 and the operation is division
        panic!("You can't divide by 0!");
    }
    return match operation_number {
        1 => first_number + second_number,
        2 => first_number - second_number,
        3 => first_number * second_number,
        4 => first_number / second_number,
        5 => first_number % second_number,
        _ => 0,
    };
}

fn main() {
    // turn the string into a number
    let number1 = get_input("Enter the first number: ");
    let number2 = get_input("Enter the second number: ");
    let operation_number = get_input("Enter the operation number: (1: +, 2: -, 3: *, 4: /, 5: %)");

    println!(
        "{} {} {} = {}",
        number1,
        get_sign(operation_number),
        number2,
        calculate_result(number1, number2, operation_number)
    );
}
