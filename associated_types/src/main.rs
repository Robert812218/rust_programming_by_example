fn main() {
    use std::ops::Add;

    impl Add<Point> for Point {
        type Output = Point;

        fn add(self, point: Point) ->  Self::Output {
            Point {
                x: self.x + point.x,
                y: self.y + point.y,
            }
        }
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1 + p2;
    }
}
