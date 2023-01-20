extern crate movies-lib;
use movies_lib::movies::play;
fn main() {
    println!("inside main of test ");
    play(“harry potter".to_string());
}
