use std::fs;
fn main() {
    
    let contents = fs::read_to_string("./text.txt").unwrap(); 
    println!("{}",contents);
}

