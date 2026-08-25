#[derive(Debug, PartialEq)]
pub struct Pila<T, const SIZE: usize> {
    a: [T; SIZE],
    cima: usize
}


impl<T: Default + Copy, const SIZE: usize> Pila<T, SIZE> {
    pub fn new() -> Pila<T, SIZE> {
        Pila { a: [T::default(); SIZE], cima: 0 }
    }


    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.cima == self.a.len() {
            return Err(String::from("Pila llena"));
        }
        self.a[self.cima] = item;
        self.cima += 1;
        Ok(())
    }


    pub fn pop(&mut self) -> Result<T, String> {
        if self.is_empty() {
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1;
        Ok(self.a[self.cima])
    }


    pub fn is_empty(&self) -> bool {
        self.cima == 0
    }
}
