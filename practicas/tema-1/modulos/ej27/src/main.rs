
use cola::Cola;
use pila:Pila;
use tokens::next_tokens;


mod tokens;
mod cola;
mod pila;
mod read_file;
mod cola;
mod pila;
mod tokens;

fn eval(e: String) -> Result<f64, String> {   //Genera una cola de tokens
    let mut tokens: Cola<&str, SIZE: 50> = Cola::new();
    let mut start: usize = 0;
    while let Some((begin: usize, end: usize)) = next_token(s: &e, ,start) {
        let _ = tokens.enqueue(item: &e[begin..=end]);
        start = end + 1;
    }
}

fn main() {
    let expressions = read_file::read("expressions.txt").unwrap();

    for e in expressions {
        println!("{}", eval(e).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn test_0() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[0].clone()), Ok(28.0));
    }

    #[test]
    fn test_1() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[1].clone()), Ok(-140.0));
    }

    #[test]
    fn test_2() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[2].clone()), Ok(13.0));
    }

    #[test]
    fn test_3() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[3].clone()), Ok(104.0));
    }

    #[test]
    fn test_4() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[4].clone()), Ok(158.4));
    }

    #[test]
    fn test_5() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(
            super::eval(expressions[5].clone()),
            Err(String::from("Incorrect expression"))
        );
    }

    #[test]
    fn test_6() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[6].clone()), Ok(-2.0));
    }

    #[test]
    fn test_7() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[7].clone()), Ok(-7.0));
    }

    #[test]
    fn test_8() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(
            super::eval(expressions[8].clone()),
            Err(String::from("Incorrect expression"))
        );
    }

    #[test]
    fn test_9() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[9].clone()), Ok(3.0));
    }

    #[test]
    fn test_10() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[10].clone()), Ok(14.0));
    }
}


