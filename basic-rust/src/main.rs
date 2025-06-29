fn main() {
    let ans = is_even(12);
    fiboo(5);
    print_series(10);
}

// conditionals
fn is_even(num: i32) ->bool{
    return num % 2 == 0;
}

fn fiboo(n: i32) -> (){
    if n>1 {
        let mut first=0;
        let mut second = 1;
        let mut i=2;
        print!("{} {}", first, second);
        while i < n { 
            let third = first+second;
            print!(" {}", third);
            first = second;
            second = third;
            i += 1;
        }
    }else{
        println!("Enter a valid number");
    }
}

// for loop
fn print_series(num: u32) -> (){
    println!();
    for i in 1..=num  {
        print!("{} ", i);
    }
}