//Q2
//stack
//yes
//copy
fn main() {

    let a = 42;
    let b = 1;
    let s = sum(a,b);

    println!("Sum of {} and {} is {}.", a, b, s);
    
    }
    
    fn sum(x: i32, y: i32) -> i32 {
        x+y
    }
