fn takesOwnership(s: String) {
    println!(" {} ", s);
}

fn makesCopy(x: i32) {
    println!(" {} ", x);
}

fn takesAndGiveBack(s: String) -> String {
    s
}

fn givesOwnership() -> String {
    let someString = String::from("Yours");
    someString
}

fn main() {
    
    // let s = String::from("Hello");
    // let s1 = s;
    // println!(" {} ", s); //s moved to s1, s doesn't have copy trait
    
    let s = String::from("Hello");
    
    takesOwnership(s); //Ownership moved here
    // println!("{} World!", s);
    
    let x = 5;
    
    makesCopy(x);
    println!(" {} ", x);
    
    //Returns value and scope
    let s1 = givesOwnership();
    let s2 = String::from("Hello");
    
    let s3 = takesAndGiveBack(s2);
    
    println!(" {}, {} ", s1, s3);
    
}