pub trait BubbleSort<T: Ord + Copy> {
    fn bubble_sort(&mut self);
}

impl<T: Ord + Copy> BubbleSort<T> for [T] {
    #[inline]
    fn bubble_sort(&mut self) {
        for i in 0..self.len() {
            for j in 1..(self.len() - i) {
                if self[j - 1] > self[j] {
                    let tmp = self[j - 1];
                    self[j - 1] = self[j];
                    self[j] = tmp;
                }
            }
        }
    }
}
