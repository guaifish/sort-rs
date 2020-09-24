use crate::insertion_sort::InsertionSort;
use crate::utils::min_max;

pub trait BucketSort<T> {
    fn bucket_sort(&mut self, bucket_size: usize);
}

macro_rules! impl_bucket_sort {
    ($ty:ty) => {
        impl BucketSort<$ty> for [$ty] {
            #[inline]
            fn bucket_sort(&mut self, bucket_size: usize) {
                // 计算数组的最大值最小值, 用于计算桶的数量, 每个数据存放的位置
                let [min, max] = min_max(self);
                // 桶的数量
                let bucket_count = (max - min) as usize / bucket_size + 1;
                // 创建空桶
                let mut buckets = vec![vec![]; bucket_count];

                // 将每个数据按照函数计算的结果放入对应的桶中
                for item in self.iter() {
                    let i = (item - min) as usize / bucket_size;
                    buckets[i].push(*item);
                }

                // 在每个桶内排序, 然后按桶的次序将数据放回数组
                let mut index = 0; // 定位已经排好序的元素位置
                for item in buckets.iter_mut() {
                    // 这里桶的内部使用插入排序
                    item.insertion_sort();

                    // 将这个桶内的数据取回数组
                    let item_len = item.len();
                    for i in 0..item_len {
                        self[index + i] = item[i];
                    }
                    index += item_len;
                }
            }
        }
    };
}

impl_bucket_sort!(i32);
impl_bucket_sort!(usize);
