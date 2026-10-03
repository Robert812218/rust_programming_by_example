use std::collections::HashMap;

fn main() {
    macro_rules! hash {
        ($( $key:expr => $value:expr ),*) => {{
            let mut hashmap = HashMap::new();

            $(
                hashmap.insert($key, $value);
            )*

            hashmap
        }};
    }

    let hashmap = hash! {
        "one" => 1,
        "two" => 2,
    };

    println!("{?}", hashmap);
}
