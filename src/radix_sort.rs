use crate::utils::{copy_slice, get_digits, max};

pub trait RadixSort {
    fn radix_sort(&mut self);
}

macro_rules! impl_radix_sort {
    ($ty:ty) => {
        impl RadixSort for [$ty] {
            #[inline]
            fn radix_sort(&mut self) {
                let n = get_digits(max(self) as usize); // 最大值的位数
                let mut buckets = vec![vec![]; 10]; // 创建10个空桶

                // 重复入桶出桶 n 次
                for i in 0..n {
                    // 根据第 i 位数字的值 j 放入对应的桶中
                    for &value in self.iter() {
                        let j = value as usize / 10_usize.pow(i as u32) % 10;
                        buckets[j].push(value);
                    }

                    // 将桶中的数据取回
                    let mut index = 0;
                    for bucket in buckets.iter_mut() {
                        copy_slice(self, bucket, index);
                        index += bucket.len();
                        bucket.clear(); // 清空桶内数据, 以便下次入桶
                    }
                }
            }
        }
    };
}

impl_radix_sort!(u8);
impl_radix_sort!(u16);
impl_radix_sort!(u32);
impl_radix_sort!(u64);
impl_radix_sort!(usize);
