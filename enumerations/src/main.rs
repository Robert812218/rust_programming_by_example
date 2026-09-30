fn main() {
    enum Expr {
        Null,
        Add(i32, i32),
        Sub(i32, i32),
        Mul(i32, i32),
        Div { dividend: i32, divisor: i32 },
        Val(i32),
    }

    let quotient = Expr::Div { dividend: 10, divisor: 2 };
    let sum = Expr::Add(40, 2);
}
