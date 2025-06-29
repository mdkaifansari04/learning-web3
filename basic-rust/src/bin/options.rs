fn get_index_of_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let string = String::from("kjdkfjdkjfd");
    let ans = get_index_of_first_a(string);
    match ans {
        Some(index) => print!("The first index a is : {}", index),
        None => print!("There is no a in the given string"),
    }
}
