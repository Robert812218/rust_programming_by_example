fn main() {
    let array = [1, 2, 3, 4];
    let array: [i16; 4] = [1, 2, 3, 4];
    let array2 = [1u8, 2, 3, 4];

    // println!("{}", array[4]); will trigger a panic because 4 index is one past the end of the
    // array
    let arrray3 = [0u8; 100];

    fn first<T>(slice: &[T]) -> &T {
        &slice[0]
    }
    println!("{}", first(&array);
    println!("{}", first(&array[2..]));
}
