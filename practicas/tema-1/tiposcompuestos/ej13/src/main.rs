fn sum<const N: usize>(a: &[u8], b: &[u8]) -> [u8; N] {
    assert!(N == a.len() + 1 || N == b.len() + 1);
    let mut result = [0; N]; // Ve almacenando en esta variable la suma

    // NO CAMBIES NADA EN LAS LÍNEAS DE ARRIBA
    let mut carry = 0; // Variable para almacenar el acarreo

        // Iteramos de derecha a izquierda
        for i in 0..N-1 {
            // Índices desde el final hacia el principio para los arrays a y b
            let a_digit = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
            let b_digit = if i < b.len() { b[b.len() - 1 - i] } else { 0 };

            // Suma de los dígitos correspondientes más el acarreo
            let sum = a_digit + b_digit + carry;
            result[N - 1 - i] = sum % 10; // Guardamos el dígito (resto de la división por 10)
            carry = sum / 10; // Calculamos el acarreo (división entera por 10)
        }

        // Si al final hay un acarreo, lo guardamos en la posición más alta
        result[0] = carry;

        result
    }



#[test]
fn no_carry_1() {
    assert_eq!([0, 5, 7, 9], sum(&[1, 2, 3], &[4, 5, 6]));
}

#[test]
fn carry_1() {
    assert_eq!([0, 5, 8, 8], sum(&[1, 2, 9], &[4, 5, 9]));
}

#[test]
fn carry_2() {
    assert_eq!([1, 4, 9, 8], sum(&[9, 9, 9], &[4, 9, 9]));
}

#[test]
fn carry_3() {
    assert_eq!([0, 5, 9, 8], sum(&[9, 9], &[4, 9, 9]));
}

#[test]
fn carry_4() {
    assert_eq!([1, 0, 4, 9, 8], sum(&[9, 9, 9, 9], &[4, 9, 9]));
}

#[test]
fn carry_5() {
    assert_eq!([1, 0, 0, 9], sum(&[9, 9, 9], &[1, 0]));
}





//fn main() {
    //let a = [1,2,3,4,5,6,7];
    //let b = [1,2,3,4,5,6,7];
    //const N: usize = 7;
    //sum::<{N}>(&a, &b);
//}

fn main() {

}