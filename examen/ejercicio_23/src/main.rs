// Es como el ejercicio anterior, pero ahora la pila es genérica
//serán genéricos tanto el tipo de los elementos como el tamaño de la pila

//Para crear un Struct genérico, se utiliza una estructura de diamante
#[derive(PartialEq, Debug)]
struct Pila <T, const SIZE: usize> { //Tipo genérico T, tamaño como una constante: usize
    a: [T; SIZE],
    cima: usize,
}


//El rasgo DEFAULT determina que deben tener un constructor por defecto -> INICIALIZADOR POR DEFECTO
//Los elementos del array de tipo T tendrían que tener el TRAAIT(rasgo) de que se puedan COPIAR
impl <T: Default + Copy, const SIZE: usize> Pila <T, SIZE> { //Hay que especificar los tipos de Pila Genérica
                                   //Los elementos de mi pila necesitan: implementar el TRAIT(rasgo) DEFAULT para los elementos de mi pila
                                   //y poder inicializarse
    fn new() -> Pila <T, SIZE> {
        Pila { a: [T::default(); SIZE], cima: 0 } // Se llama a la FUNCIÓN DEFAULT, para INICIALIZAR los elementos a 0
    }

fn push(&mut self, data: T) -> Result<(), String> {
        if self.cima == self.a.len() {  //si la cima ha llegado al final (longitud total) del array, devuelves PILA LLENA, no puedes meter más valores
            return Err(String::from("Pila llena"));
        }
        self.a[self.cima] = data; //si no está llena, guardas el dato u32 en la posición de la cima (CIMA APUNTA AL LUGAR DONDE GUARDAS PROX VALOR EN EL ARRAY)
        self.cima += 1; //después de guardar el valor incrementas el valor de la cima, para que apunte a la siguiente posición
        Ok(()) // se devuelve un RESULT del tipo unit value, CONSTRUCTOR OK
    }
    fn pop(&mut self) -> Result<T, String> {
        if self.cima == 0 { //si no hay nada en la pila, la PILA ESTÁ VACÍA
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1; //Le resto al puntero cima una unidad, para apuntar a la posición que guardó el último valor
        Ok(self.a[self.cima]) //devuelvo el valor que tenga tal cual en esa posición a la que se apunta ahora
    }

}




#[test]
fn test_push_string() {
    let mut p : Pila<&str,4> = Pila::new();
    let _ = p.push("hola");

    assert_eq!(p, Pila {a:["hola", "", "", ""], cima:1});
}

#[test]
fn test_new() {
    let p : Pila<u32,5> = Pila::new();

    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_push_if_not_full() {
    let mut p : Pila<f64,4> = Pila::new();
    let _ = p.push(3.0);

    assert_eq!(p, Pila {a:[3.0,0.0,0.0,0.0], cima:1});
}

#[test]
fn test_push_until_full() {
    let mut p : Pila<u32,4> = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);

    assert_eq!(p.push(7), Err(String::from("Pila llena")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6], cima:4});
}


#[test]
fn test_pop_if_not_empty() {
    let mut p: Pila<i32,5> = Pila::new();
    let _ = p.push(-3);

    let e = p.pop();

    assert_eq!(Ok(-3), e);
    assert_eq!(p, Pila {a:[-3,0,0,0,0], cima:0});
}

#[test]
fn test_pop_if_empty() {
    let mut p: Pila<u8,5> = Pila::new();

    let e = p.pop();

    assert_eq!(Err(String::from("Pila vacía")), e);
    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_pop_until_empty() {
    let mut p: Pila<u64,5> = Pila::new();
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