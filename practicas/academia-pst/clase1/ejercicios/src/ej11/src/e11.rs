/// Devuelve la longitud de la secuencia comenzando por `n`.
fn seq_length(mut n: i32) -> u32 {
    let mut count = 0;
    loop {
        count += 1;
        if n == 1{
            break;
        } else {
            if n % 2 == 0 { //par
                n /= 2; // n = n / 2
            } else {
                n *= 3 + 1; // n = n * 3 + 1;
            }
        }
    }
    count

}

#[test]
fn test_seq_length() {
    assert_eq!(seq_length(11), 15);
}

#[test]
fn test_seq_length_2() {
    assert_eq!(seq_length(9), 20);
}