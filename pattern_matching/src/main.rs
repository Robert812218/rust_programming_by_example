fn main() {
    fn print_expr(expr: Expr) {
        match expr {
            Expr::Null => println!("No value"),
            Expr::Add(x, y) => println!("{}", x + y),
            Expr::Sub(x, y) => println!("{}", x - y),
            Expr::Mul(x, y) => println!("{}", x * y),
            Expr::Div { dividend: x, divisor: 0 } => println!("Divisor is zero"),
            Expr::Div { dividend x, divisor: y } println!("{}", x/y),
            Expr::Val(x) => println!("{}", x),
        }
    }
}
