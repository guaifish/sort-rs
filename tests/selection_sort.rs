use rand::prelude::*;
use sorted::SelectionSort;

#[test]
fn test_selection_sort_vec() {
    let mut v = vec![6, 5, 3, 1, 8, 7, 2, 4];
    v.selection_sort();
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(v, expected);
}

#[test]
fn test_selection_sort_array() {
    let mut a = [6, 5, 3, 1, 8, 7, 2, 4];
    a.selection_sort();
    let expected = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(a, expected);

    let mut a = ['f', 'e', 'c', 'a', 'b', 'd'];
    a.selection_sort();
    let expected = ['a', 'b', 'c', 'd', 'e', 'f'];
    assert_eq!(a, expected);
}

#[test]
fn test_selection_sort_slice() {
    let mut a = [6, 5, 3, 1, 8, 7, 2, 4];
    let s: &mut [i32] = &mut a[..];
    s.selection_sort();
    let expected = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(*s, expected);
}

#[test]
fn test_selection_sort_rand() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (1..82).collect();
    let expected = nums.clone();
    nums.shuffle(&mut rng);
    nums.selection_sort();
    assert_eq!(nums, expected);
}
