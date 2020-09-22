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
                let tmp = self[i];
                let mut j = i;
                while j >= gap && self[j - gap] > tmp {
                    self[j] = self[j - gap];
                    j -= gap;
                }
                self[j] = tmp;
            }
            gap /= 2;
        }
    }
}
