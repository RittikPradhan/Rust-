use std::io;

fn main() {
    
    let mut fTemp = String::new();

    io::stdin().read_line(&mut fTemp).expect("Failed to read input");
    println!("Temperature in Fahrenheit is {}", fTemp);
    
    let mut fTemperature: i32 = fTemp.trim().parse().expect("Please type a number!");
    
    let mut cTemp = (fTemperature - 32) * 5;
    
    println!("Temperature in Celsius is {}", cTemp/9);
    
    }