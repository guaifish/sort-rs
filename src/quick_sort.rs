pub trait QuickSort<T: Ord + Copy> {
    fn quick_sort(&mut self);
}

impl<T: Ord + Copy> QuickSort<T> for [T] {
    #[inline]
    fn quick_sort(&mut self) {
        let len = self.len();
        if len > 1 {
            let mut l = 0;
            let mut r = len - 1;
            while l < r {
                while l < r && self[r] >= self[0] {
                    r -= 1;
                }
                while l < r && self[l] <= self[0] {
                    l += 1;
                }
                if l < r {
                    self.swap(l, r);
                }
            }
            if l > 0 {
                self.swap(0, l);
            }
            self[..l].quick_sort();
            self[(l + 1)..].quick_sort();
        }
    }
}
