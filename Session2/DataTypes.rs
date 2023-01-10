use std::io;

fn main() {
    let x: u8 = 254;
    let xF = 2.0; //f64
    let y: f32 = 3.0; //f32
    
    println!("{} , {} , {}", x, xF, y);
    
    let sum = 2+3;
    println!("Sum: {}", sum);
    
    let diff = 95.5 - 4.3;
    println!("Diff: {}", diff);
    
    let product = 4*5;
    println!("Product: {}", product);
    
    let quotient = 56.7/32.2;
    println!("Quotient: {}", quotient);
    
    let truncated = -5/3;
    println!("Truncated {}", truncated);
    
    let remainder = 43%5;
    println!("Remainder {}", remainder);
    
    let c = 'a';
    let z: char = 'Z'; // Explicit Type Annotation
    let cat =  '😻';
    println!(" {} , {} , {} ", c, z, cat);
    
    let t = true;
    let f: bool = false; // Explicit Type Annotation
    println!(" {}, {} ", t, f);
    
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    let (val) = tup;
    
    println!(" {}, {}, {}, {:?} ", x, y, z, val);
    
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("first, second and third tuple values {} {} {}", five_hundred, six_point_four, one); 
    
    let a = [1,2,3,4,5];
    println!("Enter an array index");
    
    let mut index = String::new();
    
    io::stdin().read_line(&mut index).expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("!Input");
    println!("Value at index {} is {} ", index, a[index]);

}