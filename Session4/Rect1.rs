#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(w: u32, h: u32) -> u32 {
    w*h
}

fn areaRect(rect: Rectangle) -> u32 {
    rect.width*rect.height
}

fn main1() {
    let width = 20;
    let height = 40;
    println!("The area is {} square pixels.\n", area(width, height));
}

fn main2() {
    let rect = (10, 50);
    println!("The area is {} square pixels.\n", area(rect.0, rect.1));
}

fn main3() {
    let rect = Rectangle {
        width: 40,
        height: 40,
    };
    println!("Value of rect is {:?} ", rect);
    println!("The area is {} square pixels.\n", areaRect(rect));
}

fn main() {
    main1();
    main2();
    main3();
    
}