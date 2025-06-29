fn main() {
    let ans = is_even(12);
    if ans{
        print!("Number is even");
    }else{
        print!("Number is odd");
    }
}

fn is_even(num: i32) ->bool{
    if num % 2 == 0 {  return true;}
    return false;
}
