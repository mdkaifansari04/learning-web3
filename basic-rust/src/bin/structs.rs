use core::num;


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


struct Rect {
    width : u32,
    height: u32,
}

impl Rect {
    fn area(&self) ->u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2* self.width * self.height
    }
    // this function are the same as static fn in js, which is called by class and doesn't have its own self object.
    fn demo_static()->u32 {
        return 0;
    }
}
fn main() {
    let rect = Rect {
        width: 32,
        height : 64
    };

    println!("Area of rect : {}", rect.area());
    println!("Perimeter of rect : {}", rect.perimeter());
    println!("Demo : {}", Rect::demo_static());

    create_user(String::from("kaif"), true, String::from("kaif@demo.com"));
}