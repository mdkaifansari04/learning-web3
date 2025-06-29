

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

//string methods 
fn get_string_length(s : &str)->usize{
    s.chars().count()
}




// structs in rust
// structs are basically the objects in JS


struct User {
    active : bool,
    email : String,
    name :String,
    sign_in_count : u64,
}

fn create_user(name: String, active : bool, email: String) -> (){
    let user = User{
        active,
        name,
        email,
        sign_in_count : 1
    };
    print!("User created : {:?}", user.name);
}


fn main() {
    let _ans = is_even(12);
    fiboo(5);
    print_series(10);
    let name = String::from("Kaif"); 
    let count = get_string_length(&name);
    println!("The length of string is :{} ",count);

    create_user(String::from("kaif"), true, String::from("kaif@demo.com"));
}