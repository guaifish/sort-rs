pub trait CountingSort<T> {
    fn counting_sort(&mut self, min: T, max: T);
}

impl CountingSort<i32> for [i32] {
    #[inline]
    fn counting_sort(&mut self, min: i32, max: i32) {
        let cap = (max - min + 1) as usize;
        let mut bucket: Vec<i32> = vec![0; cap];
        for i in self.iter() {
            bucket[(i - min) as usize] += 1;
        }
        let mut index = 0_usize;
        for (j, item) in bucket.iter_mut().enumerate() {
            while *item > 0 {
                self[index] = j as i32 + min;
                index += 1;
                *item -= 1;
            }
        }
    }
}

impl CountingSort<usize> for [usize] {
    #[inline]
    fn counting_sort(&mut self, min: usize, max: usize) {
        let cap = max - min + 1;
        let mut bucket: Vec<usize> = vec![0; cap];
        for i in self.iter() {
            bucket[i - min] += 1;
        }
        let mut index = 0_usize;
        for (j, item) in bucket.iter_mut().enumerate() {
            while *item > 0 {
                self[index] = j + min;
                index += 1;
                *item -= 1;
            }
        }
    }
}
