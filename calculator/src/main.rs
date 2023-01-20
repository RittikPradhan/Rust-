use std::env::{args, Args};

fn calculate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        '%' => first_number % second_number,
        '_' => panic!("Invalid Operator Used.")
    }
}

fn main() {
    let mut args: Args = args();   
    let first: String = args.nth(1).unwrap();

    // After accessing the second argument, the iterator's next element becomes the first
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    // println!("{:?}", args);
    println!(" {} {} {} ", first_number, operator, second_number);

    // calculate(operator, first_number, second_number);
}
