/*
  Trying to implment usecases of Enum and vector 
*/

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64,f64,f64),
}

fn max_shape(v: Vec<Shape>) -> Shape {
    let mut mv : f64 = 0.0;
    let mut rs : Shape = Shape::Circle(0.0); 
    for val in v {
        let  t : f64 = match val {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c) => { let s = (a+b+c)/2.0; f64::sqrt(s*(s-a)*(s-b)*(s-c)) }  
        };
        if mv < t {
            mv = t;
            rs = val;    
        }
    };
    rs
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(8.0,4.0,5.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c) => { let s = (a+b+c)/2.0; f64::sqrt(s*(s-a)*(s-b)*(s-c))  }
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
    println!("Shape with the highest area {:?}", max_shape(shapes));
}
