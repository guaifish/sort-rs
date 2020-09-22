trait MaxHeap<T: Ord + Copy> {
    fn build_heap(&mut self);
    fn heapify(&mut self, i: usize);
}

impl<T: Ord + Copy> MaxHeap<T> for [T] {
    fn build_heap(&mut self) {
        let len = self.len() / 2;
        for i in (0..=len).rev() {
            self.heapify(i);
        }
    }

    fn heapify(&mut self, i: usize) {
        let len = self.len();
        if len > 1 {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut j = i;
            if l < len && self[l] > self[j] {
                j = l;
            }
            if r < len && self[r] > self[j] {
                j = r;
            }
            if j > i {
                self.swap(i, j);
                self.heapify(j);
            }
        }
    }
}

pub trait HeapSort<T: Ord + Copy> {
    fn heap_sort(&mut self);
}

impl<T: Ord + Copy> HeapSort<T> for [T] {
    #[inline]
    fn heap_sort(&mut self) {
        let len = self.len();
        if len > 1 {
            self.build_heap();
            for i in (1..len).rev() {
                self.swap(0, i);
                self[0..i].heapify(0);
            }
        }
    }
}
