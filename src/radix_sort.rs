pub trait RadixSort {
    fn radix_sort(&mut self);
}

impl RadixSort for [usize] {
    #[inline]
    fn radix_sort(&mut self) {
        let max = self.iter().max().unwrap();
        let max_digit = (1..).find(|&x| max / 10_usize.pow(x) == 0).unwrap();
        let mut buckets = vec![vec![]; 10];
        for d in 0..max_digit {
            for item in self.iter() {
                let i = item / 10_usize.pow(d as u32) % 10;
                buckets[i].push(*item);
            }
            let mut index = 0;
            for item in buckets.iter_mut() {
                let l = item.len();
                self[index..(index + l)].copy_from_slice(item);
                item.clear();
                index += l;
            }
        }
    }
}
