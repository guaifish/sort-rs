use crate::utils::min_max;

pub trait CountingSort {
    fn counting_sort(&mut self);
}

macro_rules! impl_counting_sort {
    ($ty:ty) => {
        impl CountingSort for [$ty] {
            #[inline]
            fn counting_sort(&mut self) {
                // 计数排序根据值的范围创建桶, 每个值一个桶
                let [min, max] = min_max(self);
                let bucket_count = (max - min + 1) as usize;
                // 创建空桶, 每个桶的初始计数为0
                let mut buckets: Vec<usize> = vec![0; bucket_count];

                // 遍历数组对每个值进行计数
                for value in self.iter() {
                    buckets[(value - min) as usize] += 1;
                }

                // 根据计数对数组进行赋值
                let mut index = 0;
                for (value, &count) in buckets.iter().enumerate() {
                    for i in 0..count {
                        self[index + i] = value as $ty + min;
                    }
                    index += count;
                }
            }
        }
    };
}

impl_counting_sort!(i8);
impl_counting_sort!(i16);
impl_counting_sort!(i32);
impl_counting_sort!(i64);
impl_counting_sort!(isize);
impl_counting_sort!(u8);
impl_counting_sort!(u16);
impl_counting_sort!(u32);
impl_counting_sort!(u64);
impl_counting_sort!(usize);
