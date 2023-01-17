struct Book {
    name:&'static str,
    id: u32
}

//trait
trait Printable {
    fn print(&self);
    fn id_value(&self);
}

impl Printable for Book {
    fn print(&self) {
        println!("Printing book with id: {} and name: {}", self.id, self.name);
    }

    fn id_value(&self) {
        println!("Curent book id is: {}", self.id);
    }
}

fn main() {

    //Instance of struct
    let b1 = Book {
        id: 1000,
        name: "Rust in Action" 
    };

    b1.print();
    b1.id_value();
}