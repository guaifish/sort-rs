pub trait BubbleSort<T: Ord + Copy> {
    fn bubble_sort(&mut self);
}

impl<T: Ord + Copy> BubbleSort<T> for [T] {
    #[inline]
    fn bubble_sort(&mut self) {
        let len = self.len();
        for i in 0..len {
            for j in 1..(len - i) {
                if self[j - 1] > self[j] {
                    self.swap(j - 1, j);
                }
            }
        }
    }
}
