struct Data<T> {
    value: T,
}

use std::fmt::Display;

fn print_fn<T: Display>(t: T) {
    println!("Inside print_fn Function");
    println!("{}", t);
}

//The <T> syntax known as the type parameter, is used to declare a generic construct. T represents any data-type

fn main() {

    let t1: Data<i32> = Data { value: 1000 };
    println!("Value is: {}", t1.value);

    let s1: Data<String> = Data { value: "Hello World".to_string() };
    println!("Value is: {}", s1.value);

    print_fn(10 as u8);
    print_fn(20 as u16);
    print_fn(true as bool);
    print_fn("Hello World");

}