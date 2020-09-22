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
            'o: loop {
                let pivot = self[0];
                'ir: loop {
                    if l >= r {
                        break 'o;
                    }
                    if self[r] < pivot {
                        break 'ir;
                    } else {
                        r -= 1;
                    }
                }
                'il: loop {
                    if l >= r {
                        break 'o;
                    }
                    if self[l] > pivot {
                        break 'il;
                    } else {
                        l += 1;
                    }
                }
                self.swap(l, r);
            }
            if l > 0 {
                self.swap(0, l);
            }
            self[..l].quick_sort();
            self[(l + 1)..].quick_sort();
        }
    }
}
