use crate::utils::{max, get_digits, copy_slice};

pub trait RadixSort {
    fn radix_sort(&mut self);
}

impl RadixSort for [usize] {

    #[inline]
    fn radix_sort(&mut self) {
        let max_digits = get_digits(max(self)); // 最大值的位数
        let mut buckets = vec![vec![]; 10];     // 创建空桶

        // 重复入桶出桶 max_digits 次
        for i in 0..max_digits {

            // 根据第 i 位的值 j 放入对应的桶中
            for &value in self.iter() {
                let j = value / 10_usize.pow(i as u32) % 10;
                buckets[j].push(value);
            }

            // 将桶中的数据取回
            let mut index = 0;
            for bucket in buckets.iter_mut() {
                let l = bucket.len();
                self[index..(index + l)].copy_from_slice(bucket);
                // copy_slice(self, bucket, start: usize)
                bucket.clear();
                index += l;
            }
        }
    }
}
