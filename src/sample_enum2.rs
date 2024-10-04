use std::f64::consts::PI;

enum Shape {
    Square(f64), // square has an associated side
    Circle(f64), // circle has an associated radius
    Rectangle(f64, f64) // rectandle has an associated w and h
}
   

fn main() {
    let square = Shape::Square(10.0);
    print_area(square);

    let circle = Shape::Circle(10.0);
    print_area(circle);

    let rectangle = Shape::Rectangle(10.0, 20.0);
    print_area(rectangle);
    

}

fn print_area(shape: Shape) -> f64 {
    // pattern matching
    let area = match shape {
        Shape::Square(side) => side * side,
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(w, h) => w * h
    };

    println!("Area {}", area);

    return area;

}
