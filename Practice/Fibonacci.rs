use std::io;

fn main() {

    let mut N = String::new();
    
    io::stdin().read_line(&mut N).expect("Failed to read input");
    
    let mut n: u32 = N.parse().expect("!Input");
    
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    
    println!("The fibonacci series till {} is : ", n);
    print!(" {} {} ", a, b);
    
    while(c<=n) {
        print!(" {} ", c);
        a=b; b=c; c=a+b;
    }
}