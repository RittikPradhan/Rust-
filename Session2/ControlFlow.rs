use std::io;

fn main() {

    let mut N = String::new();
    
    io::stdin().read_line(&mut N).expect("Failed at read line");
    let n: u32 = N.trim().parse().expect("!Input");
    
    if n % 5 == 0 {
        println!("{} is divisible by 5", n);
    }
    else if n % 2 == 0 {
         println!("{} is divisible by 2", n)
    }
    else {
        println!("{} isn't divisible", n);
    }
    
    //loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(" Result: {}", result);
    
    //nested loops
    
    let mut C = 0;
    
    'countingUp: loop {
        println!("Count: {}", C);
        
        let mut remaining = 10; 
        
        loop {
            println!("Remaining: {}", remaining);
            
            if remaining == 5 {
                break;
            }
            if C == 2 {
                break 'countingUp;
            }
            remaining -= 1;
        }
        C +=1;
    }
    
    println!(" Final Count: {} ", C);
    
    //whileLoop

    let mut num = 10;
    
    while num !=0 {
        println!("While {} ", num);
        num -= 2;
    }
    
    //forIn loop
    let a = [1,2,3,4,5];
    
    for i in a {
        print!(" {} ", i);
    }
    
    //countdown
    for i in (1..4).rev() {
        println!(" {} ", i);
    }
}