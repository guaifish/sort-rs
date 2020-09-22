pub trait MergeSort<T: Ord + Copy> {
    fn merge_sort(&mut self);
}

impl<T: Ord + Copy> MergeSort<T> for [T] {
    fn merge_sort(&mut self) {
        let len = self.len();
        if len > 1 {
            let mid = len / 2;
            self[..mid].merge_sort();
            self[mid..].merge_sort();
            let mut l = 0;
            let mut r = mid;
            while l < r && r < len {
                if self[r] < self[l] {
                    self[l..=r].rotate_right(1);
                    r += 1;
                }
                l += 1;
            }
        }
    }
}
