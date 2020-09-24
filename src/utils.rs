#[allow(dead_code)]
pub fn min_max<T: Ord + Copy>(slice: &[T]) -> [Option<T>; 2] {
    let len = slice.len();
    if len > 0 {
        let mut min = slice[0];
        let mut max = slice[0];
        for item in slice[1..].iter() {
            if *item > max {
                max = *item;
            }
            if *item < min {
                min = *item;
            }
        }
        [Some(min), Some(max)]
    } else {
        [None, None]
    }
}

#[test]
fn test_min_max_empty_input() {
    let [min, max] = min_max::<i32>(&[]);
    assert_eq!(min, None);
    assert!(max.is_none());
}

#[test]
fn test_min_max_one_ele() {
    let [min, max] = min_max(&[-2]);
    assert_eq!(min.unwrap(), -2);
    assert_eq!(max, Some(-2));

    let [min, max] = min_max(&[10_usize]);
    assert_eq!(min, Some(10));
    assert_eq!(max.unwrap(), 10);
}

#[test]
fn test_min_max_multi_ele() {
    let [min, max] = min_max(&[-2, 0, 4, -5, 3]);
    assert_eq!(min.unwrap(), -5);
    assert_eq!(max, Some(4));

    let [min, max] = min_max(&['h', 'e', 'l', 'l', 'o']);
    assert_eq!(min, Some('e'));
    assert_eq!(max.unwrap(), 'o');
}
