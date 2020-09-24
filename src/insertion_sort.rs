use crate::utils::insert;

pub trait InsertionSort<T: Ord + Copy> {
    fn insertion_sort(&mut self);
}

impl<T: Ord + Copy> InsertionSort<T> for [T] {
    #[inline]
    fn insertion_sort(&mut self) {
        for i in 1..self.len() {
            insert(self, i, 1);
        }
    }
}
