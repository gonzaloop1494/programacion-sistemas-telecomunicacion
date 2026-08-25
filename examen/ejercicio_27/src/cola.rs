#[derive(Debug, PartialEq)] //TRAITS Debug (para imprimir en modo depuración la información de forma legible) y
//TRAIT PartialEq (para que se puedan comparar entre sí dos colas)
pub struct Cola<T, const SIZE: usize> {
    a: [T; SIZE],
    first: usize, //contamos desde 0 en adelante (NÚMERO DE ELEMENTOS)
    last: usize, //tienen índices usize para indexar elementos de un array
    len: usize
}

//Al implementar, el tipo de elemento T debe tener la capacidad de copiar elementos por defecto y la capacidad de copiarse
impl<T: Copy + Default, const SIZE: usize> Cola<T, SIZE> {
    pub fn new() -> Cola<T, SIZE> {
        Cola { a: [T::default(); SIZE], first: 0, last: 0, len: 0 }
    }
    //CONSTRUCTOR new para un struct genérico
    pub fn len(&self) -> usize {
        self.len
    }


    pub fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.len == SIZE {
            return Err(String::from("Cola llena"));
        }
        self.a[self.last] = item;
        self.last = if self.last == self.a.len() - 1 { 0 } else { self.last + 1 };
        self.len += 1;
        Ok(())
    }


    pub fn dequeue(&mut self) -> Result<T, String> {
        if self.len == 0 {
            return Err(String::from("Cola vacía"));
        }
        let item = self.a[self.first];
        self.first = if self.first == self.a.len() - 1 { 0 } else { self.first + 1 };
        self.len -= 1;
        Ok(item)
    }
}