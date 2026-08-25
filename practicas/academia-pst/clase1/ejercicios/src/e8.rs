fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a; //a
    } else {
        return b; //b
    }
    // Esta función debe devolver el valor del parámetro que sea mayor
    // Si son iguales devuelve cualquiera de ellos
    // No puedes usar:
    // - ni llamadas a otras funciones
    // - ni nuevas variables
    todo!()
}

//////////////////////////////////////
//
// NO MODIFIQUES NADA A PARTIR DE AQUÍ
//
#[test]
fn ten_is_bigger_than_eight() {
    assert_eq!(10, bigger(10, 8));
}

#[test]
fn fortytwo_is_bigger_than_thirtytwo() {
    assert_eq!(42, bigger(32, 42));
}

#[test]
fn equal_numbers() {
    assert_eq!(42, bigger(42, 42));
}