// siempre que pase un array a un programa,
// hay que pasarlo por referencia
//&mut es una referencia exclusiva, nos permite modificar el array
fn min_index_desde(a: &[i8], from: usize) -> usize {
    let mut min: usize = from;
    for index in (from + 1)..a.len() {
        if a[index] < a[min] {
            min = index;
        }
        min
    }
}
fn swap_items(mut n1: i8, mut n2: i8) {
    let aux: i8 = n1;
    n1 = n2;
}
fn partition(a: &mut [i8]) {
    for index  in 0..(a.len() - 1) {
        let min = min_index_desde(a,from);
        if min != index {
            swap_items(a[min] > a[index])
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
    println!("Hello, world!");
}
