pub trait InsertionSort<T: Ord + Copy> {
    fn insertion_sort(&mut self);
}

impl<T: Ord + Copy> InsertionSort<T> for [T] {
    fn insertion_sort(&mut self) {
        for i in 1..self.len() {
            if self[i] < self[i - 1] {
                let mut j = i;
                while j > 0 && self[j - 1] > self[i] {
                    j -= 1;
                }
                self[j..=i].rotate_right(1);
            }
        }
    }
}
