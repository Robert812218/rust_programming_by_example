#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;

    println!("{}", p1.x);
    print_point(&p2);
}

fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}
