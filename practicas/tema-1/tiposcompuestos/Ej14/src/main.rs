fn search(t: i32, a: &[i32]) -> isize {
    for (index, &item) in a.iter().enumerate() {
        if item == t {
            return index as isize;
        }
    }
    -1
}



#[test]
fn middle_odd() {
    assert_eq!(2, search(3, &[14, 25, 3, -5, -2]));
}

#[test]
fn middle_even_1() {
    assert_eq!(3, search(3, &[14, -5, -2, 3, 7, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(2, search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, search(42, &[]));
}

fn main() {}
