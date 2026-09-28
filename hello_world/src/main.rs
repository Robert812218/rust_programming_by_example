fn main() {
    let name: &str = "world";
    println!("Hello, {}!", name);

    let mut age = 42;
    age += 1;

    let mut a = 15;
    let mut b = 40;
    while b != 0 {
        let temp = b;
        a = temp;
    }
    println!("Greatest common divisor of 15 and 40 is: {}", a);

    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }
    #[derive(Debug)]
    let point = Point {
        x: 24,
        y: 42,
    };
    println!("{:?}", point);
}
