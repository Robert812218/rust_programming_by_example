fn main() {
    fn max<T: PartialOrd>(a: T, b: T) -> {
        if a > b {
            a
        } else {
            b
        }
        println!("{}", max('a', 'z'));
    }

    enum Option<T> {
        Some(T),
        None,
    }
}
