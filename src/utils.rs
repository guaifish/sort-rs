#![allow(dead_code)]

pub fn min_max<T: Ord + Copy>(slice: &[T]) -> [T; 2] {
    // 保证 slice != []
    let mut min = slice[0];
    let mut max = slice[0];
    for &item in slice[1..].iter() {
        if item > max {
            max = item;
        }
        if item < min {
            min = item;
        }
    }
    [min, max]
}

pub fn max<T: Ord + Copy>(slice: &[T]) -> T {
    // 保证 slice != []
    let mut max = slice[0];
    for &item in slice[1..].iter() {
        if item > max {
            max = item;
        }
    }
    max
}

pub fn min<T: Ord + Copy>(slice: &[T]) -> T {
    // 保证 slice != []
    let mut min = slice[0];
    for &item in slice[1..].iter() {
        if item > min {
            min = item;
        }
    }
    min
}

pub fn get_digits(num: usize) -> usize {
    // 获得一个十进制数字的位数
    let mut digits = 1;
    loop {
        if num / 10_usize.pow(digits) == 0 {
            break digits as usize;
        } else {
            digits += 1;
        }
    }
}

pub fn copy_slice<T: Copy>(slice: &mut [T], src: &[T], start: usize) {
    // 将 src 的数据复制到 slice 中, 从 start 位置开始
    // 保证不会超出 slice 索引范围
    for (i, &item) in src.iter().enumerate() {
        slice[start + i] = item;
    }
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

#[test]
fn test_get_digits() {
    assert_eq!(get_digits(3), 1);
    assert_eq!(get_digits(2378), 4);
    assert_eq!(get_digits(10_usize.pow(10)), 11);
}

#[test]
fn test_copy_slice() {
    let mut a = [1, 2, 3, 4, 5, 6];
    copy_slice(&mut a, &[7, 8], 3);
    assert_eq!(a, [1, 2, 3, 7, 8, 6]);

    copy_slice(&mut a, &[0; 3], 1);
    assert_eq!(a, [1, 0, 0, 0, 8, 6]);
}
