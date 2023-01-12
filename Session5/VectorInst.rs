fn main() {
    
    let mut v1 = Vec::new();
    v1.push(1); v1.push(2); v1.push(3);
    
    println!("Length of Vector is {} ", v1.len());
    println!(" {:?} ", v1);
    
    
    let mut v2 = vec!['a','b','c','d'];
    println!("Length of Vector is {} ", v2.len());
    println!(" {:?} ", v2);
    
    v2.remove(1);
    println!(" {:?} ", v2);
    
    if v2.contains(&'c') {
        println!("Found");
    }
    
}