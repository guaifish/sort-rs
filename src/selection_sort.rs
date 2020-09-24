pub trait SelectionSort<T: Ord + Copy> {
    fn selection_sort(&mut self);
}

impl<T: Ord + Copy> SelectionSort<T> for [T] {
    #[inline]
    fn selection_sort(&mut self) {
        let len = self.len();
        for i in 0..len {
            let mut k = i;
            for j in (i + 1)..len {
                if self[j] < self[k] {
                    k = j;
                }
            }
            if i != k {
                self.swap(i, k);
            }
        }
    }
}
