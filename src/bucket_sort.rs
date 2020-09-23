use crate::insertion_sort::InsertionSort;

pub trait BucketSort<T> {
    fn bucket_sort(&mut self, bucket_size: usize);
}

impl BucketSort<i32> for [i32] {
    fn bucket_sort(&mut self, bucket_size: usize) {
        let len = self.len();
        let mut min = self[0];
        let mut max = self[0];
        for i in 1..len {
            if self[i] < min {
                min = self[i];
            } else if self[i] > max {
                max = self[i];
            }
        }
        let bucket_count = (max - min) as usize / bucket_size + 1;
        let mut buckets = vec![vec![]; bucket_count];
        for i in 0..len {
            buckets[(self[i] - min) as usize / bucket_size].push(self[i]);
        }
        let mut index = 0;
        for i in 0..buckets.len() {
            buckets[i].insertion_sort();
            for j in 0..buckets[i].len() {
                self[index] = buckets[i][j];
                index += 1;
            }
        }
    }
}

#[test]
fn test_bucket_sort_vec() {
    let mut v = vec![6, 5, 3, 1, 8, 7, 2, 4, -7, -10, 0, 6, 1];
    v.bucket_sort(3);
    let expected = vec![-10, -7, 0, 1, 1, 2, 3, 4, 5, 6, 6, 7, 8];
    assert_eq!(v, expected);

    let mut v: Vec<i32> = vec![6, 5, 3, 1, 8, 7, 2, 4];
    v.bucket_sort(3);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(v, expected);
}
