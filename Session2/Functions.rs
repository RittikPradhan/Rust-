fn five() -> u32 {
    5
}

fn increment(x: u32) -> u32 {
    x+1
}

fn main() {
    let x = five();
    
    println!(" {} ", x);
    
    let x = increment(x);
    
    println!(" {} ", x);
}