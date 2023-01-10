#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }
    
    fn canHold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 50,
    };
    
    println!("The area is {} ", rect.area());
    
    let rect1 = Rectangle {
        width: 40,
        height: 80,
    };
    
    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.canHold(&rect2));
}