use std::io;
// use rand::Rng; //Rng is a trait
// use std::cmp::Ordering; //Ordering is enum with three values: "Less", "Greater", "Equal"

fn main() {
    
    //loop {
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read Input");
    
    let mut guess: u32 = guess.trim().parse().expect("!Input");
    
    // let secretNumber: u32 = secretNumber = rand::thread_rng().gen_range(1..=100);
    let SECRET_NUMBER = 99;
    
    //check(guess, secretNumber);
    check(guess, SECRET_NUMBER);
    
    /*
    match guess.cmp(&secretNumber) {
        Ordering::Less => println!("Too Small!");
        Ordering::Greater => println!("Too Big!");
        Ordering::Equal => {
            println!("CorrectGuess!!");
            break;
        }
    }
    }
    */
}

fn check(guess: u32, secretNumber: u32) {
    if(guess == secretNumber) {
        println!("Correct Guess");
    }
    else {
        println!("Incorrect Guess");
    }
}

//Actual Code :
/*
use std::io;
use rand::Rng; // Rng is a trait
use std::cmp::Ordering; //Ordering is enum with three values: "Less", "Greater" "Equal"
fn main() {
    println!("guessing Game!!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);// gen_range is defined in Rng Trait
    println!("The secret number is: {secret_number}");

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        println!("you guessed : {guess}");
    
        let mut guess: u32 = guess.trim().parse().expect("Please type a number!");// trim removes whitespace entered by user while giving input
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }// end of ordering
           } //end of match
       
    }// end of loop
} // end of main
*/
