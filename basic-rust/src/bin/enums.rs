use core::num;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East, 
    West
}


enum Shape {
    Rectangle(f64, f64),
    Square(f64),
    Circle(f64),
}

fn main(){
    let direction = Direction::West;
    print!("Direction : {:?}", direction);

    let rect = Shape::Rectangle(12.1, 3.4);
    calculate_area(rect);

    let circle = Shape::Circle(12.1);
    calculate_area(circle);

    let sq = Shape::Square(78.0);
    calculate_area(sq);

}   

fn calculate_area(shape: Shape) -> f64{
    match shape {
        Shape::Circle(r) => 3.14 * r *r,
        Shape::Rectangle(a,b ) => a*b,
        Shape::Square(a)=> a*a
    }
}