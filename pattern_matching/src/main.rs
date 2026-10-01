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

    fn uppercase(c: u8) -> u8 {
        match c {
            b'a'...b'z' => c - 32,
            _ => c,
        }
    }

    println!("{}", uppercase(b'a') as char;

    fn is_alphanumeric(c: char) -> bool {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => true,
            _ => false,
        }
    }

    fn uppercase(c: u8) -> u8 {
        if let b'a' ... b'z' = c {
            c - 32
        } else {
            c 
        }
    }
}
