fn main() {
    impl Point {
        fn dist_from_origin(&self) -> f64 {
            let sum_of_squares = self.x.pow(2) + self.y.pow(2);
            (sum_of_squares as f64).sqrt()
        }
    }

    impl Point {
        fn translate(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y = dy;
        }
    }
}
