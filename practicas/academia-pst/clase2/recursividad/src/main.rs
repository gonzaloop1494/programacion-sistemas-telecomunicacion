fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 4;
    factorial(n);
    println!("El factorial de 4 es: {}", factorial(n));
}
