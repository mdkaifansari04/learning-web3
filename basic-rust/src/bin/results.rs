use std::fs;

fn main() {
    let result = fs::read_to_string("input.txt");
    match result {
        Ok(content) => println!("Content : {:?}", content),
        Err(err) => print!("Error : {:?}", err),
    }
}
