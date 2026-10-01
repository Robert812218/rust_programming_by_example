fn main() {
    trait BitSet {
        fn clear(&mut self, index: usize);
        fn is_set(&self, index: usize) -> bool;
        fn set(&mut self, index: usize);

        fn toggle(&mut self, index: usize) {
            if self.is_set(index) {
                self.clear(index);
            } else {
                self.set(index);
            }
        }
    }

    impl BitSet for u64 {
        fn toggle(&mut self, index: usize) {
            *self^= 1 << index;
        }
    }
}
