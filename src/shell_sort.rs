use crate::utils::insert;

pub trait ShellSort<T: Ord + Copy> {
    fn shell_sort(&mut self);
}

impl<T: Ord + Copy> ShellSort<T> for [T] {
    #[inline]
    fn shell_sort(&mut self) {
        let len = self.len();
        let mut gap = len / 2;
        while gap > 0 {
            for i in gap..len {
                insert(self, i, gap);
            }
            gap /= 2;
        }
    }
}
