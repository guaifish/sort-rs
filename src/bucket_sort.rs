use crate::insertion_sort::InsertionSort;

pub trait BucketSort<T> {
    fn bucket_sort(&mut self, bucket_size: usize);
}

impl BucketSort<i32> for [i32] {
    #[inline]
    fn bucket_sort(&mut self, bucket_size: usize) {
        let min = self.iter().min().unwrap();
        let max = self.iter().max().unwrap();
        let bucket_count = (max - min) as usize / bucket_size + 1;
        let mut buckets = vec![vec![]; bucket_count];
        for item in self.iter() {
            let i = (item - min) as usize / bucket_size;
            buckets[i].push(*item);
        }
        let mut index = 0;
        for item in buckets.iter_mut() {
            item.insertion_sort();
            let l = item.len();
            self[index..(index + l)].copy_from_slice(item);
            index += l;
        }
    }
}

impl BucketSort<usize> for [usize] {
    #[inline]
    fn bucket_sort(&mut self, bucket_size: usize) {
        let min = self.iter().min().unwrap();
        let max = self.iter().max().unwrap();
        let bucket_count = (max - min) / bucket_size + 1;
        let mut buckets = vec![vec![]; bucket_count];
        for item in self.iter() {
            let i = (item - min) / bucket_size;
            buckets[i].push(*item);
        }
        let mut index = 0;
        for item in buckets.iter_mut() {
            item.insertion_sort();
            let l = item.len();
            self[index..(index + l)].copy_from_slice(item);
            index += l;
        }
    }
}
