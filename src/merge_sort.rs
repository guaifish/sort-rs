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
            let mut head = 0;
            let mut l = 0;
            let mut r = mid;
            while l < mid && r < len {
                if self[head] > self[r] {
                    let tmp = self[r];
                    let mut i = r;
                    while i > head {
                        self[i] = self[i - 1];
                        i -= 1;
                    }
                    self[head] = tmp;
                    r += 1;
                } else {
                    l += 1;
                }
                head += 1;
            }
        }
    }
}
