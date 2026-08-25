fn bin_search(t: i32, a: &[i32]) -> isize {
    let mut left: isize = 0;
    let mut right: isize = (a.len()  as isize) - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid as usize] == t{
            return mid;
        } else if t < a[mid as usize] {
            right = mid - 1;
        } else {    //t > a[mid]
            left = mid + 1;
        }
    }
    -1
}

#[test]
fn middle_odd() {
    assert_eq!(2, bin_search(3, &[-5, -2, 3, 14, 25]));
}

#[test]
fn middle_even_1() {
    assert_eq!(2, bin_search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(3, bin_search(7, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, bin_search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, bin_search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, bin_search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, bin_search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, bin_search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, bin_search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, bin_search(42, &[]));
}

fn main() {}