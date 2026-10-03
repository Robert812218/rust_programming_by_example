use std::ops::{Add, Sub};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    macro_rules! op {
        (+ $self:ident : $self_type ty, $other:indent, $expr:expr) => {
            impl Add for $self_type {
                type Output = $self_type;

                fn add(self, $other: $self_type) -> self.type {
                    $expr
                }
            }
        };

        (- $self:ident : $self_type:ty, $other:ident $expr: expr) => {
            impl Sub for $self_type {
                type Output = $self_type;

                fn sub(self, $other: $self_type) -> $self_type {
                    $expr
                }
            }
        }
    }

    op!(+ self: Point, other {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    })

    let a = Point { x: 10, y: 20 };
    let b = Point { x: 3, y: 5 };

    let sum = a + b;

    println!("{:?}", sum);
}
