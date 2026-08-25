#[derive(Debug, PartialEq)]
struct Pila<T, const SIZE: usize> {
    a: [T; SIZE],
    cima: usize
}

impl <T: Default + Copy, const SIZE: usize> Pila <T, SIZE> {
    fn new() -> Pila<T, SIZE> {

        Pila { a: [T::default(); SIZE], cima: 0 }
    }

    fn push(&mut self, value: T) -> Result<(), String> {
        if self.cima == self.a.len() {
            return Err(String::from("Pila llena"));
        }
        self.a[self.cima] = value;
        self.cima += 1;
        Ok(())
    }
    fn pop(&mut self) -> Result<T, String> {
        if self.cima == 0 {
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1;
        Ok(self.a[self.cima])
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

