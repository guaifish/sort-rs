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
            let mut c = 0;
            let mut l = 0;
            let mut r = mid;
            while l < r && r < len {
                while r < len && self[r] < self[l] {
                    r += 1;
                    c += 1;
                }
                if c == 0 {
                    l += 1;
                } else {
                    self[l..r].rotate_right(c);
                    l += c;
                    c = 0;
                }
            }
        }
    }
}
