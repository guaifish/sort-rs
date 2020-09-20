pub trait InsertionSort<T: Ord + Copy> {
    fn insertion_sort(&mut self);
}

impl<T: Ord + Copy> InsertionSort<T> for [T] {
    fn insertion_sort(&mut self) {
        for i in 0..self.len() {
            let tmp = self[i];
            let mut index = i;
            while index > 0 && self[index - 1] > tmp {
                self[index] = self[index - 1];
                index -= 1;
            }
            self[index] = tmp;
        }
    }
}
