pub trait InsertionSort<T: Ord + Copy> {
    fn insertion_sort(&mut self);
}

impl<T: Ord + Copy> InsertionSort<T> for [T] {
    #[inline]
    fn insertion_sort(&mut self) {
        for i in 1..self.len() {
            let mut j = i;
            let tmp = self[i];
            while j > 0 && self[j - 1] > tmp {
                self[j] = self[j - 1];
                j -= 1;
            }
            self[j] = tmp;
        }
    }
}
