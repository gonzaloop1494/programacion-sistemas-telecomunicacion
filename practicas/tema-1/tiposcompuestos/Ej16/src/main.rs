fn rec_bin_search(t: i32, a: &[i32]) -> isize {
    rec_bin_search_aux(t, a, 0, (a.len() as isize) - 1)
}
fn rec_bin_search_aux(t: i32, a: &[i32], left: isize, right: isize) -> isize {
    if left <= right {
        let mid = ( left + right ) / 2;
        if a[mid as usize] == t {
            mid
        } else if t < a[mid as usize] {
            rec_bin_search_aux(t, a, left, mid - 1)
        } else { // t > a[mid]
            rec_bin_search_aux(t, a, mid + 1, right)
        }

    } else {
        -1
    }
}

#[test]
fn middle_odd() {
    assert_eq!(2, rec_bin_search(3, &[-5, -2, 3, 14, 25]));
}

#[test]
fn middle_even_1() {
    assert_eq!(2, rec_bin_search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(3, rec_bin_search(7, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, rec_bin_search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, rec_bin_search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, rec_bin_search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, rec_bin_search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, rec_bin_search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, rec_bin_search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, rec_bin_search(42, &[]));
}

fn main() {}