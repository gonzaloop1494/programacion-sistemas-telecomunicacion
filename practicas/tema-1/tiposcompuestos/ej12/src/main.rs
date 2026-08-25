fn partition(a: &mut [i8]) {
    let mut left = 0;
    let mut right = a.len() as isize - 1;

    while left <= right {
        if a[left as usize] < 0 {
            left += 1;
        } else if a[right as usize] >= 0 {
            right -= 1;
        } else {
            a.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }
    }
}

#[test]
fn all_tests() {
    let test_cases = vec![
        vec![1, 2, 0, -1, -2, 1, -4, 2],
        vec![-5, 1, 2, 0, -1, -2, 1, -4, 2],
        vec![-5, 1, 2, 0, -1, -2, 1, -4, 2, -8],
        vec![-1, 2, 0, -1, -2, 1, -4],
        vec![1, 2, 0],
        vec![-1, -2, -3],
    ];

    for mut orig in test_cases {
        let mut array = orig.clone();

        partition(&mut array);
        eprintln!("test case: {orig:?}\nYour result:    {array:?}\n");

        // check array is partitioned: negatives on the left
        for i in 0..array.len() {
            if i < array.len() - 1 {
                assert!(!(array[i] >= 0 && array[i + 1] < 0))
            };
        }

        // check the partitioned array has the same elements as orig
        array.sort();
        orig.sort();
        assert_eq!(array, orig);
    }
}

fn main() {
    partition(&mut [-1, 2, 1, -4, 2]);
    println!("partition result: {:?}", &partition(&mut [-1, -1, 2, -4, 2]));
}