pub trait SelectionSort<T: Ord + Copy> {
    fn selection_sort(&mut self);
}

impl<T: Ord + Copy> SelectionSort<T> for [T] {
    #[inline]
    fn selection_sort(&mut self) {
        for i in 0..self.len() {
            let mut k = i;
            for j in (i + 1)..self.len() {
                if self[k] >= self[j] {
                    k = j;
                }
            }
            self.swap(i, k);
        }
    }
}
