enum Shape {
    Rect,
    Circle
}
   

fn main() {
    let shape = Shape::Rect;
    print_area(shape);

}

fn print_area(shape: Shape) {
    println!("Hi There")
}
