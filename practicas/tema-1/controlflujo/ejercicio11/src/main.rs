/// Devuelve la longitud de la secuencia comenzando por `n`.
fn seq_length(mut n: i32) -> u32 {
    let mut total: u32 = 0;
    loop {
        total += 1; // total = total + 1;
        if n == 1 {
            break;
        } else {
            if n % 2 == 0 { // Par
                n /= 2; // n = n / 2;
            } else { // Impar
                n = n * 3 + 1;
            }
        }
    }
    total
}
// let mut total: u32 = 0;
//while n != 1 {
  //  total += 1;
    //n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
//}
//total += 1;
//total




//if n == 1 {
//  1
// } else if n % 2 == 0 {
//  1 + seq_length( n / 2 )
// } else {
//  1 + seq_length( 3 * n + 1 )
// }

#[test]
fn test_seq_length() {
    assert_eq!(seq_length(11), 15);
}

#[test]
fn test_seq_length_2() {
    assert_eq!(seq_length(9), 20);
}

fn main() {
    let n1 : i32 = 5;
    println!("La longitud de la secuencia para {} es : {}", n1, seq_length(n1));
}