fn main() {

    let numbers = [1, 2, 3, 4, 5];
    println!("array = {:?}", numbers); //[1, 2, 3, 4, 5]
        
    let slice = &numbers[1..3];
    println!("slice = {:?}", slice); //[2, 3]

    let slice = &numbers[..3];
    println!("slice = {:?}", slice); //[1, 2, 3]

    let slice = &numbers[2..];
    println!("slice = {:?}", slice); //[3, 4, 5]

    let slice = &numbers[..];
    println!("slice = {:?}", slice); //[1, 2, 3, 4, 5]


    //mutable slice
    let mut colors = ["red", "green", "yellow", "white"];
    println!("array = {:?}", colors); //["red", "green", "yellow", "white"]

    
    let sliced_colors = &mut colors[1..3];
    println!("original slice = {:?}", sliced_colors); //["green", "yellow"]

    sliced_colors[1] = "purple";
    println!("changed slice = {:?}", sliced_colors); //["green", "purple"]

    // slice the string
    let string = String::from("Hello World!");

    let slice = &string[0..5];

    println!("string = {}", string); //Hello World!
    println!("slice = {}", slice); //Hello
}