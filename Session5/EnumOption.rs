
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

fn _plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => Some(i+1).unwrap()
    }
}

fn main() {

    let one = Some(1);
    let two = plus_one(one);
    let none = plus_one(None);

    let three = _plus_one(None);
    // let three = _plus_one(Some(2147483647)); //thread 'main' panicked at 'attempt to add with overflow', note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    println!("{:?} {:?} {:?} {:?}", one, two, none, three);
}