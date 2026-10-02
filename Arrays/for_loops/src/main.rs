fn main() {
    let array = [1, 2, 3, 4];
    let mut sum = 0;
    for element in &array {
        // * = go to the element that * is referencing 
        sum += *element;
    }
    println!("{}", sum);

    // usize means it can return ether no value (None) or the index(Some(index))
    fn index<T: PartialEq(slice: &[T], target &T) -> Option<usize> {
        for (index, element) in slice.iter().enumerate() {
            if element == target {
                return Some(index);
            }
        }
        None
    }

    fn min_max(slice: &[i32]) -> Option<(i32, i32)> {
        if slice.is_empty() {
            return None;
        }
        let mut min = slice[0];
        let mut max = slice[0];
        for &element in  slice {
            if element < min {
                min = element;
            }
            if element > max {
                max = element;
            }
        }
        Some((min, max))
    }
}
