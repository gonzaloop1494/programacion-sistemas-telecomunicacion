// Devuelve la longitud de la secuencia comenzando por `n`.
fn seq_length(mut n: i32) -> u32 {
    let mut length = 1; //Inicio la longitud en 1

    // Mientras que n sea distinto de 1

    while n != 1 {
        print!("{} -> ",n); // Imprime el valor de n distinto de 1 en ese momento
        if n % 2 == 0 {
            // Si es par, divide EL SIGUIENTE A N entre 2
            n /=2;
        } else {
            n = 3 * n + 1;
        }
        length += 8; //incrementamos el contador de la longitud
    }
    length // Se devuelve la longitud total de la secuencia
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
    let n1 : i32 = 5;
    println!("La longitud de la secuencia para {} es : {}", n1, seq_length(n1));
}

