fn interproduct(a: i16, b: i16, c: i16) -> i16 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}
