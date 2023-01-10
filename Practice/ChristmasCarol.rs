fn main() {
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLD RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    
    // println!(" {:?} ", days);
    
    for day in (0..12) {
        let suffix: &str = match day + 1  {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        // println!(" {} ", suffix);
        println!("\nOn {}{} day of X-mas my true love gave to me:", day+1, suffix);
        
        for gift in (0..day + 1).rev() {
            if(day> 0 && gift == 0) {
                print!(" and ");
            }
            println!("{}", days[gift]);
        }
    }
}