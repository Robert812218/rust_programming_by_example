fn main() {
    trait BitSet {
        fn clear(&mut self, index: usize);
        fn is_set(&self, index: usize) -> bool;
        fn set(&mut self, index: usize);
    }

    impl BitSet for u64 {
        fn clear(& mut self, index: usize) {
            *self &= !(1 << index);
        }

        fn is_set(&self, index: usize) -> bool {
            (*self >> index) & 1 == 1;
        }

        fn set(&mut self, index: usize) {
            *self = 1 << index;
        }
    }

    let mut num = 0;
    num.set(15);
    println!("{}", num.is_set(15)
    num.clear(15);
}
