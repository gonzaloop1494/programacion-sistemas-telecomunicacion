pub fn bin_search(t: i32, a: &[i32]) -> Option<usize> {
    let mut left: isize = 0;
    let mut right: isize = (a.len() as isize) - 1;
    while left <= right {
        let middle = (left + right) / 2;
        if a[middle as usize] == t {
            return Some(middle as usize);
        } else if t < a[middle as usize] {
            right = middle - 1;
        } else { // t > a[middle]
            left = middle + 1;
        }
    }
    None
}
