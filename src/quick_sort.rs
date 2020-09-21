pub trait QuickSort<T: Ord + Copy> {
    fn quick_sort(&mut self);
}

impl<T: Ord + Copy> QuickSort<T> for [T] {
    fn quick_sort(&mut self) {
        let len = self.len();
        if len > 1 {
            let mut left = 0;
            let mut right = len - 1;
            while left < right {
                while left < right && self[right] >= self[0] {
                    right -= 1;
                }
                while left < right && self[left] <= self[0] {
                    left += 1;
                }
                self.swap(left, right);
            }
            self.swap(0, left);
            self[..left].quick_sort();
            self[(left + 1)..].quick_sort();
        }
    }
}
