use rand::prelude::*;
use sorted::BucketSort;

#[test]
fn test_bucket_sort_vec() {
    let mut v = vec![6, 5, 3, 1, 8, 7, 2, 4, -7, -10, 0, 6, 1];
    v.bucket_sort(4);
    let expected = vec![-10, -7, 0, 1, 1, 2, 3, 4, 5, 6, 6, 7, 8];
    assert_eq!(v, expected);

    let mut v: Vec<usize> = vec![6, 5, 3, 1, 8, 7, 2, 4];
    v.bucket_sort(4);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(v, expected);
}
#[test]
fn test_bucket_sort_array() {
    let mut v = [6, 5, 3, 1, 8, 7, 2, 4, -7, -10, 0, 6, 1];
    v.bucket_sort(4);
    let expected = [-10, -7, 0, 1, 1, 2, 3, 4, 5, 6, 6, 7, 8];
    assert_eq!(v, expected);

    let mut v: [usize; 8] = [6, 5, 3, 1, 8, 7, 2, 4];
    v.bucket_sort(4);
    let expected = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(v, expected);
}

#[test]
fn test_bucket_sort_slice() {
    let mut a = [6, 5, 3, 1, 8, 7, 2, 4];
    let s: &mut [i32] = &mut a[..];
    s.bucket_sort(4);
    let expected = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(*s, expected);
}

#[test]
fn test_bucket_sort_rand() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (-100..=100).flat_map(|x| [x].repeat(3)).collect();
    let expected = nums.clone();
    nums.shuffle(&mut rng);
    nums.bucket_sort(10);
    assert_eq!(nums, expected);
}
