pub trait QuickSort<T: Ord + Copy> {
    fn quick_sort(&mut self);
}

impl<T: Ord + Copy> QuickSort<T> for [T] {
    #[inline]
    fn quick_sort(&mut self) {
        // 1. 取第一个值作为基准值, 左右两个坐标 l 和 r 往中间靠
        // 2. 左边遇到大于基准值时停止, 右边遇到小于基准值时停止
        // 3. 交换左右两边的值然后继续重复此过程直到 l >= r
        // 4. 交换基准值与停止位置的值, 此时基准值左边的值都小于基准值, 右边的值都大于基准值
        // 5. 将左边和右边的数组继续递归排序
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
            if l != 0 {
                self.swap(0, l);
            }
            self[..l].quick_sort();
            self[(l + 1)..].quick_sort();
        }
    }
}
