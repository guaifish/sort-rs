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
                let [min, max] = min_max(self);
                let min = min.unwrap();
                let max = max.unwrap();
                let bucket_count = (max - min) as usize / bucket_size + 1;
                let mut buckets = vec![vec![]; bucket_count];
                for item in self.iter() {
                    let i = (item - min) as usize / bucket_size;
                    buckets[i].push(*item);
                }
                let mut index = 0;
                for item in buckets.iter_mut() {
                    item.insertion_sort();
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
