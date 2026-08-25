#[derive(Debug, PartialEq)]
struct Pila {
    a: [u32; 5],
    cima: usize
}
impl Pila {
    fn new() -> Self{
        Pila { a: [0,0,0,0,0], cima: 0 }
    }
    fn push(&mut self, value: u32) -> Result<(), String> {
     if self.cima == self.a.len() {
         return Err(String::from("Pila llena"));
     }
     self.a[self.cima] = value;
     self.cima += 1;
     return Ok(());
    }
    fn pop(&mut self) -> Result<u32, String> {
        if self.cima == 0 {
            return Err(String::from("Pila vacía"));
        }
        let value = self.a[self.cima - 1];
        self.cima -= 1;
        Ok((value))
        //self.cima -=1;
        //Ok((self.a.cima))
    }
}
#[test]
fn test_new() {
    let p = Pila::new();

    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_push_if_not_full() {
    let mut p = Pila::new();
    let _ = p.push(3); // Push devuelve un valor de tipo result,
    // utilizo el valor anónimo _

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

    let e = p.pop();

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