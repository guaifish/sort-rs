pub trait InsertionSort<T: Ord + Copy> {
    fn insertion_sort(&mut self);
}

impl<T: Ord + Copy> InsertionSort<T> for [T] {
    #[inline]
    fn insertion_sort(&mut self) {
        for index in 1..self.len() {
            // 不断往左移, 直至找到一个比 self[index] 小的位置可供插入
            for i in (0..index).rev() {
                // 将 index 位置的值放到 i 位置, i 到 index - 1 的值往右移
                if self[i] <= self[index] {
                    let tmp = self[index];
                    for j in (i..index).rev() {
                        self[j + 1] = self[j];
                    }
                    self[i] = tmp;
                }
            }
        }
    }
}
