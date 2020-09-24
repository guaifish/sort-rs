#[allow(dead_code)]
pub fn min_max<T: Ord + Copy>(slice: &[T]) -> [T; 2] {
    // 保证 slice.len() != 0
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
    [min, max]
}

#[test]
fn test_min_max() {
    let [min, max] = min_max(&[2]);
    assert_eq!(min, 2);
    assert_eq!(max, 2);

    let [min, max] = min_max(&[-2, 0, 4, -5, 3]);
    assert_eq!(min, -5);
    assert_eq!(max, 4);

    let [min, max] = min_max(&['h', 'e', 'l', 'l', 'o']);
    assert_eq!(min, 'e');
    assert_eq!(max, 'o');
}
