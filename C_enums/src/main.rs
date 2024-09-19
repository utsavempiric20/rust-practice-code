enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let ans = calculate_shape_area(Shape::Rectangle(2.5, 3.5));
    println!("Rectangle {}", ans);
    let ans1 = calculate_shape_area(Shape::Circle(5.5));
    println!("Circle {}", ans1)
}

fn calculate_shape_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
