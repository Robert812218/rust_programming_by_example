struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let point1 = Point::new(24, 42);
    let point2 = Point::origin();

    println!("({}, {})", point1.x, point1.y);
    println!("({}, {})", point2.x, point2.y);

    let tuple = (24, 42);
    println!("({}, {})", tuple.0, tuple.1);

    let (hello, world) = "helloworld".split_at(5);
    println!("{}, {}!", hello, world);
}
