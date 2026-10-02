fn main() {
    macro_rules! int_bitset {
        // the macro's pattern
        ($ty:ty) => {
            impl BitSet for $ty {
                fn clear(&mut self, index:usize) {
                    // *self: get the actual integer we're modifying
                    // &= is the bitwise AND assignment operator
                    // << is the left shift operator which shifts the bits to the left
                    *self &= (1 << index);
                }
                // defines a method called is_set 
                // *self gets the integer and >> shifts its bits to the right 
                fn is_set(&self, index: usize) -> bool {
                    // Checks if the extracted bit is 1
                    (*self >> index) & 1 == 1
                }

                fn set(&mut self, index: usize) {
                    // Creates a value with one bit set
                    // index = 0 
                    // 0001
                    // // 
                    // index = 1
                    // 0010
                    // //
                    // index = 2
                    // 0100
                    // // 
                    // index = 3
                    // 1000
                    *self = 1 << index;
                }
            }
        }
    }
}
