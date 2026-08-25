fn sum<const N: usize>(a: &[u8], b: &[u8]) -> [u8; N] {
    assert!(N == a.len() + 1 || N == b.len() + 1);
    let mut result = [0; N]; // Ve almacenando en esta variable la suma

    let mut offset: usize = 0;
    let mut acarreo: u8 = 0;
    while offset < a.len() || offset < b.len() {
        let digit = acarreo
            + if offset < a.len() {a[a.len() - 1 - offset]} else {0}
    }
}

fn main() {
    println!("Hello, world!");
}
