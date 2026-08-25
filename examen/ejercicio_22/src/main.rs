#[derive(PartialEq, Debug)]
struct Pila {
    a: [u32; 5],
    cima: usize,
}

impl Pila { //la función new no recibe ningún ingrediente (lo vemos en los test), y devuelve un valor de tipo Pila (Self)
    fn new() -> Pila {
        Pila {a: [0; 5], cima: 0} //inicializamos a cero cada campo del Self creado
    }
    fn push(&mut self, data: u32) -> Result<(), String> {
        if self.cima == self.a.len() {  //si la cima ha llegado al final (longitud total) del array, devuelves PILA LLENA, no puedes meter más valores
            return Err(String::from("Pila llena"));
        }
        self.a[self.cima] = data; //si no está llena, guardas el dato u32 en la posición de la cima (CIMA APUNTA AL LUGAR DONDE GUARDAS PROX VALOR EN EL ARRAY)
        self.cima += 1; //después de guardar el valor incrementas el valor de la cima, para que apunte a la siguiente posición
        Ok(()) // se devuelve un RESULT del tipo unit value, CONSTRUCTOR OK
    }
    fn pop(&mut self) -> Result<u32, String> {
        if self.cima == 0 { //si no hay nada en la pila, la PILA ESTÁ VACÍA
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1; //Le resto al puntero cima una unidad, para apuntar a la posición que guardó el último valor
        Ok(self.a[self.cima]) //devuelvo el valor que tenga tal cual en esa posición a la que se apunta ahora
    }

}


#[test]
fn test_new() {
    let p = Pila::new();

    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});  //aquí se puede intuir que el tipo Pila va a ser un STRUCT, con los campos a(array) y cima, que es usize
// el operador == de Rust no sabe comparar nuestro tipo de datos, hay que indicarle que es posible comparar valores de ese tipo de datos
    //TRAIT -> #[derive(PartialEq)] -> Tipos que puedan contener valores de tipo real
    //TRAIT -> DEBUG -> Para pintar mejor valores de nuestro tipo creado
}

#[test]
fn test_push_if_not_full() {
    let mut p = Pila::new();
    let _ = p.push(3);

    assert_eq!(p, Pila {a:[3,0,0,0,0], cima:1});
}

#[test]
fn test_push_until_full() {
    let mut p = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);
    let _ = p.push(7);

    assert_eq!(p.push(8), Err(String::from("Pila llena")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6,7], cima:5});
}


#[test]
fn test_pop_if_not_empty() {
    let mut p = Pila::new();
    let _ = p.push(3);

    let e = p.pop(); //Pop tiene dos constructores: Ok(en este caso i32) y Err(en este caso tipo String)

    assert_eq!(Ok(3), e);
    assert_eq!(p, Pila {a:[3,0,0,0,0], cima:0});
}

#[test]
fn test_pop_if_empty() {
    let mut p = Pila::new();

    let e = p.pop();

    assert_eq!(Err(String::from("Pila vacía")), e);
    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_pop_until_empty() {
    let mut p = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);
    let _ = p.push(7);


    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();

    assert_eq!(p.pop(), Err(String::from("Pila vacía")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6,7], cima:0});
}

fn main() {}