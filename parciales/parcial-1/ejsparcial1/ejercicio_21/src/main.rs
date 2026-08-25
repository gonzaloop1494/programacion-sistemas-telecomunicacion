 #[derive(Copy, Clone)]

struct Square{
    width: i32
}


impl Square {
    fn area(&self) -> i32 {  // el &self me permite un uso más eficiente de la memoria, sin el clon
                             // automático (paso una referencia a mi variable)
        self.width * self.width
    }
}

#[test]
fn t() {
    let s = Square { width: 2 };
    let s2 = s;

    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());

    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());
}


fn main() {
    println!("Hello, world!");
}
