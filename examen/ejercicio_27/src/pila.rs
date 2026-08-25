#[derive(PartialEq, Debug)]
pub struct Pila <T, const SIZE: usize> { //Tipo genérico T, tamaño como una constante: usize
    a: [T; SIZE],
    cima: usize,
}


//El rasgo DEFAULT determina que deben tener un constructor por defecto -> INICIALIZADOR POR DEFECTO
//Los elementos del array de tipo T tendrían que tener el TRAAIT(rasgo) de que se puedan COPIAR
impl <T: Default + Copy, const SIZE: usize> Pila <T, SIZE> { //Hay que especificar los tipos de Pila Genérica
    //Los elementos de mi pila necesitan: implementar el TRAIT(rasgo) DEFAULT para los elementos de mi pila
    //y poder inicializarse
    pub fn new() -> Pila <T, SIZE> {
        Pila { a: [T::default(); SIZE], cima: 0 } // Se llama a la FUNCIÓN DEFAULT, para INICIALIZAR los elementos a 0
    }

    pub fn push(&mut self, data: T) -> Result<(), String> {
        if self.cima == self.a.len() {  //si la cima ha llegado al final (longitud total) del array, devuelves PILA LLENA, no puedes meter más valores
            return Err(String::from("Pila llena"));
        }
        self.a[self.cima] = data; //si no está llena, guardas el dato u32 en la posición de la cima (CIMA APUNTA AL LUGAR DONDE GUARDAS PROX VALOR EN EL ARRAY)
        self.cima += 1; //después de guardar el valor incrementas el valor de la cima, para que apunte a la siguiente posición
        Ok(()) // se devuelve un RESULT del tipo unit value, CONSTRUCTOR OK
    }
    pub fn pop(&mut self) -> Result<T, String> {
        if self.cima == 0 { //si no hay nada en la pila, la PILA ESTÁ VACÍA
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1; //Le resto al puntero cima una unidad, para apuntar a la posición que guardó el último valor
        Ok(self.a[self.cima]) //devuelvo el valor que tenga tal cual en esa posición a la que se apunta ahora
    }
    pub fn is_empty(&self) -> bool {
        self.cima == 0
    }
}


