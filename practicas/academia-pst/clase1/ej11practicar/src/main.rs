/// Devuelve la longitud de la secuencia comenzando por `n`.
fn seq_length(mut n: i32) -> u32 {
    let mut total = 0;
    loop {
        total += 1;
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }
    total
}

#[test]
fn test_seq_length() {
    assert_eq!(seq_length(11), 15);
}

#[test]
fn test_seq_length_2() {
    assert_eq!(seq_length(9), 20);
}

fn main() {
    let n1 = 10;
    println! ("La longitud de la secuencia de n = {}, es : {}", n1, seq_length(n1));

}